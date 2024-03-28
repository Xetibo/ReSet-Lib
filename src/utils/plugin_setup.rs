use std::{
    fs::create_dir,
    io::ErrorKind,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use dbus_crossroads::{Crossroads, IfaceBuilder, IfaceToken};
use libloading::Library;
use once_cell::sync::Lazy;

use crate::{create_config, write_log_to_file, ErrorLevel, ERROR};

use super::{
    dbus_utils::DBUS_PATH,
    plugin::{PluginCapabilities, PluginImplementation, PluginTestFunc, SidebarInfo},
};

pub static mut FRONTEND_PLUGINS: Lazy<Vec<FrontendPluginFunctions>> = Lazy::new(|| {
    SETUP_LIBS();
    setup_frontend_plugins()
});
pub static mut BACKEND_PLUGINS: Lazy<Vec<BackendPluginFunctions>> = Lazy::new(|| {
    SETUP_LIBS();
    setup_backend_plugins()
});
static mut LIBS: Vec<libloading::Library> = Vec::new();
static mut PLUGIN_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));

static SETUP_PLUGIN_DIR: fn() -> Option<PathBuf> = || -> Option<PathBuf> {
    let config = create_config("Xetibo", "ReSet").expect("Could not create config directory");
    let plugin_dir = create_dir(config.join("plugins"));
    if let Err(error) = plugin_dir {
        if error.kind() != ErrorKind::AlreadyExists {
            ERROR!("Failed to read plugin directory", ErrorLevel::Critical);
            None
        } else {
            Some(config.join("plugins"))
        }
    } else {
        Some(config.join("plugins"))
    }
};

static SETUP_LIBS: fn() = || {
    let read_dir: fn(PathBuf) = |dir: PathBuf| {
        let plugin_dir = dir.read_dir().expect("Could not read directory");
        plugin_dir.for_each(|plugin| {
            if let Ok(file) = plugin {
                unsafe {
                    let path = file.path();
                    let lib = libloading::Library::new(&path);
                    if let Ok(lib) = lib {
                        LIBS.push(lib);
                    } else {
                        ERROR!(
                            format!(
                                "File was not a library! Please delete the faulty file: {}",
                                path.to_str().unwrap()
                            ),
                            ErrorLevel::Recoverable
                        );
                    }
                }
            }
        });
    };
    let plugin_dir = SETUP_PLUGIN_DIR();
    unsafe {
        if PLUGIN_DIR.is_dir() {
            read_dir(PLUGIN_DIR.clone());
        } else if let Some(plugin_dir) = plugin_dir {
            read_dir(plugin_dir)
        }
    }
};

fn setup_backend_plugins() -> Vec<BackendPluginFunctions> {
    let mut plugins = Vec::new();
    unsafe {
        for lib in LIBS.iter() {
            let capabilities = get_plugin_capabilities(lib);
            if capabilities.is_none() {
                break;
            }
            let capabilities = capabilities.unwrap();
            match capabilities.get_implementation() {
                PluginImplementation::Both => (),
                PluginImplementation::Backend => (),
                PluginImplementation::Frontend => break,
            }
            let dbus_interface: Result<
                libloading::Symbol<unsafe extern "C" fn(Arc<RwLock<CrossWrapper>>)>, // -> Plugin>,
                libloading::Error,
            > = lib.get(b"dbus_interface");
            let startup: Result<
                libloading::Symbol<unsafe extern "C" fn() -> ()>,
                libloading::Error,
            > = lib.get(b"backend_startup");
            let shutdown: Result<
                libloading::Symbol<unsafe extern "C" fn() -> ()>,
                libloading::Error,
            > = lib.get(b"backend_shutdown");
            let name: Result<
                libloading::Symbol<unsafe extern "C" fn() -> String>,
                libloading::Error,
            > = lib.get(b"name");
            let tests: Result<
                libloading::Symbol<unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
                libloading::Error,
            > = lib.get(b"backend_tests");
            if let (Ok(dbus_interface), Ok(startup), Ok(shutdown), Ok(name), Ok(tests)) =
                (dbus_interface, startup, shutdown, name, tests)
            {
                plugins.push(BackendPluginFunctions::new(
                    capabilities.get_capabilities(),
                    startup,
                    shutdown,
                    name,
                    dbus_interface,
                    tests,
                ));
            } else {
                ERROR!("Failed to load plugin", ErrorLevel::Critical);
            }
        }
    }
    plugins
}

fn setup_frontend_plugins() -> Vec<FrontendPluginFunctions> {
    let mut plugins = Vec::new();
    unsafe {
        for lib in LIBS.iter() {
            let capabilities = get_plugin_capabilities(lib);
            if capabilities.is_none() {
                break;
            }
            let capabilities = capabilities.unwrap();
            match capabilities.get_implementation() {
                PluginImplementation::Both => (),
                PluginImplementation::Backend => break,
                PluginImplementation::Frontend => (),
            }
            let startup_frontend: Result<
                libloading::Symbol<unsafe extern "C" fn() -> ()>,
                libloading::Error,
            > = lib.get(b"frontend_startup");
            let shutdown_frontend: Result<
                libloading::Symbol<unsafe extern "C" fn() -> ()>,
                libloading::Error,
            > = lib.get(b"frontend_shutdown");
            let data_frontend: Result<
                libloading::Symbol<unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>)>,
                libloading::Error,
            > = lib.get(b"frontend_data");
            let tests_frontend: Result<
                libloading::Symbol<unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
                libloading::Error,
            > = lib.get(b"frontend_tests");
            if let (
                Ok(startup_frontend),
                Ok(shutdown_frontend),
                Ok(data_frontend),
                Ok(tests_frontend),
            ) = (
                startup_frontend,
                shutdown_frontend,
                data_frontend,
                tests_frontend,
            ) {
                plugins.push(FrontendPluginFunctions::new(
                    capabilities.get_capabilities(),
                    startup_frontend,
                    shutdown_frontend,
                    data_frontend,
                    tests_frontend,
                ));
            }
        }
    }
    plugins
}

fn get_plugin_capabilities(lib: &Library) -> Option<PluginCapabilities> {
    unsafe {
        let capabilities: Result<
            libloading::Symbol<unsafe extern "C" fn() -> PluginCapabilities>,
            libloading::Error,
        > = lib.get(b"capabilities");
        if capabilities.is_err() {
            ERROR!("Failed to load plugin", ErrorLevel::Critical);
            return None;
        }
        Some((capabilities.unwrap())())
    }
}

#[allow(improper_ctypes_definitions)]
pub struct BackendPluginFunctions {
    pub capabilities: Vec<&'static str>,
    pub startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
    pub data: libloading::Symbol<'static, unsafe extern "C" fn(Arc<RwLock<CrossWrapper>>)>, //-> Plugin>,
    pub tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
}

#[allow(improper_ctypes_definitions)]
impl BackendPluginFunctions {
    pub fn new(
        capabilities: Vec<&'static str>,
        backend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
        shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
        name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
        data: libloading::Symbol<'static, unsafe extern "C" fn(Arc<RwLock<CrossWrapper>>)>,
        tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
    ) -> Self {
        Self {
            capabilities,
            startup: backend_startup,
            shutdown,
            name,
            data,
            tests,
        }
    }
}

unsafe impl Send for BackendPluginFunctions {}

unsafe impl Sync for BackendPluginFunctions {}

#[allow(improper_ctypes_definitions)]
pub struct FrontendPluginFunctions {
    pub capabilities: Vec<&'static str>,
    pub frontend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_data:
        libloading::Symbol<'static, unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>)>,
    pub frontend_tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
}

#[allow(improper_ctypes_definitions)]
impl FrontendPluginFunctions {
    pub fn new(
        capabilities: Vec<&'static str>,
        frontend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
        frontend_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
        frontend_data: libloading::Symbol<
            'static,
            unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>),
        >,
        frontend_tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
    ) -> Self {
        Self {
            capabilities,
            frontend_startup,
            frontend_shutdown,
            frontend_data,
            frontend_tests,
        }
    }
}

unsafe impl Send for FrontendPluginFunctions {}
unsafe impl Sync for FrontendPluginFunctions {}

type Registration<T> = Box<dyn FnOnce(&mut IfaceBuilder<T>)>;

pub struct CrossWrapper<'a>(&'a mut Crossroads);

impl<'a> CrossWrapper<'a> {
    pub fn new(cross: &'a mut Crossroads) -> Self {
        Self(cross)
    }

    pub fn register<T: Send + Sync + 'static>(
        &mut self,
        name: impl Into<String>,
        token: Registration<T>,
    ) -> dbus_crossroads::IfaceToken<T> {
        self.0.register(name.into(), token)
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, interfaces: &[IfaceToken<T>], data: T) {
        self.0.insert(DBUS_PATH, interfaces, data);
    }
}

unsafe impl<'a> Send for CrossWrapper<'a> {}
unsafe impl<'a> Sync for CrossWrapper<'a> {}
