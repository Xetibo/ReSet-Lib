use std::{
    fs::create_dir,
    hint::spin_loop,
    io::ErrorKind,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, RwLock},
};

use dbus_crossroads::{Crossroads, IfaceToken};
use libloading::Library;
use once_cell::sync::Lazy;
use toml::Value;

use crate::{create_config_directory, ERROR, LOG};
#[cfg(debug_assertions)]
use crate::{utils::macros::ErrorLevel, write_log_to_file};

use super::{
    config::CONFIG,
    plugin::{PluginCapabilities, PluginImplementation, PluginTestFunc, SidebarInfo},
};

pub static LIBS_LOADED: AtomicBool = AtomicBool::new(false);
pub static LIBS_LOADING: AtomicBool = AtomicBool::new(false);
pub static mut FRONTEND_PLUGINS: Lazy<Vec<FrontendPluginFunctions>> = Lazy::new(|| {
    SETUP_LIBS();
    setup_frontend_plugins()
});
pub static mut BACKEND_PLUGINS: Lazy<Vec<BackendPluginFunctions>> = Lazy::new(|| {
    SETUP_LIBS();
    setup_backend_plugins()
});
static mut LIBS: Vec<libloading::Library> = Vec::new();
pub static mut PLUGIN_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));

static SETUP_PLUGIN_DIR: fn() -> Option<PathBuf> = || -> Option<PathBuf> {
    let config = create_config_directory("reset").expect("Could not create config directory");
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
    if LIBS_LOADING.load(std::sync::atomic::Ordering::SeqCst) {
        while !LIBS_LOADED.load(std::sync::atomic::Ordering::SeqCst) {
            spin_loop();
        }
        return;
    }
    LIBS_LOADING.store(true, std::sync::atomic::Ordering::SeqCst);
    let read_dir: fn(PathBuf) = |dir: PathBuf| {
        let binding = CONFIG;
        let plugins = binding.get("plugins");
        if plugins.is_none() {
            LOG!("No plugins entry found in config");
            return;
        }
        let plugins = plugins.unwrap().as_array();
        if plugins.is_none() {
            ERROR!("Wrong config, please write plugins entry as array of strings: e.g [\"libyourplugin.so\"]", ErrorLevel::PartialBreakage);
            return;
        }
        let plugins = plugins.unwrap();
        let plugin_dir = dir.read_dir().expect("Could not read directory");
        plugin_dir.for_each(|plugin| {
            if let Ok(file) = plugin {
                if !plugins.contains(&Value::String(String::from(
                    file.file_name().to_str().unwrap_or(""),
                ))) {
                    return;
                }
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
    #[allow(clippy::borrow_interior_mutable_const)]
    let plugin_dir = if let Some(config) = CONFIG.get("plugin_path") {
        let config = config.as_str();
        if config.is_none() {
            SETUP_PLUGIN_DIR()
        } else {
            let maybe_dir = PathBuf::from(config.unwrap());
            if maybe_dir.is_dir() {
                Some(maybe_dir)
            } else {
                SETUP_PLUGIN_DIR()
            }
        }
    } else {
        SETUP_PLUGIN_DIR()
    };
    unsafe {
        if PLUGIN_DIR.is_dir() {
            read_dir(PLUGIN_DIR.clone());
        } else if let Some(plugin_dir) = plugin_dir {
            read_dir(plugin_dir)
        }
    }
    LIBS_LOADED.store(true, std::sync::atomic::Ordering::SeqCst);
};

fn setup_backend_plugins() -> Vec<BackendPluginFunctions> {
    let mut plugins = Vec::new();
    unsafe {
        for lib in LIBS.iter() {
            let capabilities = get_plugin_capabilities(lib);
            if capabilities.is_none() {
                continue;
            }
            let capabilities = capabilities.unwrap();
            match capabilities.get_implementation() {
                PluginImplementation::Both => (),
                PluginImplementation::Backend => (),
                PluginImplementation::Frontend => continue,
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
                continue;
            }
            let capabilities = capabilities.unwrap();
            match capabilities.get_implementation() {
                PluginImplementation::Both => (),
                PluginImplementation::Backend => continue,
                PluginImplementation::Frontend => (),
            }
            let frontend_name: Result<
                libloading::Symbol<unsafe extern "C" fn() -> String>,
                libloading::Error,
            > = lib.get(b"frontend_name");
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

            match (
                frontend_name,
                startup_frontend,
                shutdown_frontend,
                data_frontend,
                tests_frontend,
            ) {
                (
                    Ok(frontend_name),
                    Ok(startup_frontend),
                    Ok(shutdown_frontend),
                    Ok(data_frontend),
                    Ok(tests_frontend),
                ) => {
                    plugins.push(FrontendPluginFunctions::new(
                        capabilities.get_capabilities(),
                        frontend_name,
                        startup_frontend,
                        shutdown_frontend,
                        data_frontend,
                        tests_frontend,
                    ));
                }
                (Err(error), _, _, _, _) => {
                    ERROR!(
                        format!("Failed to load plugin function: {}", error),
                        ErrorLevel::PartialBreakage
                    );
                }
                (_, Err(error), _, _, _) => {
                    ERROR!(
                        format!("Failed to load plugin function: {}", error),
                        ErrorLevel::PartialBreakage
                    );
                }
                (_, _, Err(error), _, _) => {
                    ERROR!(
                        format!("Failed to load plugin function: {}", error),
                        ErrorLevel::PartialBreakage
                    );
                }
                (_, _, _, Err(error), _) => {
                    ERROR!(
                        format!("Failed to load plugin function: {}", error),
                        ErrorLevel::PartialBreakage
                    );
                }
                (_, _, _, _, Err(error)) => {
                    ERROR!(
                        format!("Failed to load plugin function: {}", error),
                        ErrorLevel::PartialBreakage
                    );
                }
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
        capabilities: (Vec<&'static str>, bool),
        backend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
        shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
        name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
        data: libloading::Symbol<'static, unsafe extern "C" fn(Arc<RwLock<CrossWrapper>>)>,
        tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
    ) -> Self {
        Self {
            capabilities: capabilities.0,
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
    pub capabilities: (Vec<&'static str>, bool),
    pub frontend_name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
    pub frontend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_data:
        libloading::Symbol<'static, unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>)>,
    pub frontend_tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
}

#[allow(improper_ctypes_definitions)]
impl FrontendPluginFunctions {
    pub fn new(
        capabilities: (Vec<&'static str>, bool),
        frontend_name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
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
            frontend_name,
            frontend_startup,
            frontend_shutdown,
            frontend_data,
            frontend_tests,
        }
    }
}

unsafe impl Send for FrontendPluginFunctions {}

unsafe impl Sync for FrontendPluginFunctions {}

pub struct CrossWrapper<'a>(&'a mut Crossroads);

impl<'a> CrossWrapper<'a> {
    pub fn new(cross: &'a mut Crossroads) -> Self {
        Self(cross)
    }

    pub fn register<T: Send + Sync + 'static>(
        &mut self,
        name: impl Into<String>,
        token: fn(&mut dbus_crossroads::IfaceBuilder<T>),
    ) -> dbus_crossroads::IfaceToken<T> {
        self.0.register(name.into(), token)
    }

    pub fn insert<T: Send + Sync + 'static>(
        &mut self,
        object_name: impl Into<String>,
        interfaces: &[IfaceToken<T>],
        data: T,
    ) {
        self.0.insert(
            "/org/Xebito/ReSet/Plugins/".to_string() + &object_name.into(),
            interfaces,
            data,
        );
    }
}

unsafe impl<'a> Send for CrossWrapper<'a> {}

unsafe impl<'a> Sync for CrossWrapper<'a> {}
