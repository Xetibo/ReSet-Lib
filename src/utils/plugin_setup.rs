use std::{fs::create_dir, io::ErrorKind, path::PathBuf};

use dbus_crossroads::Crossroads;
use once_cell::sync::Lazy;

use crate::{create_config, ERROR, ErrorLevel, write_log_to_file};

use super::plugin::{PluginCapabilities, PluginTestFunc, SidebarInfo};

pub static mut PLUGINS: Lazy<Vec<PluginFunctions>> = Lazy::new(|| {
    SETUP_LIBS();
    SETUP_PLUGINS()
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

static SETUP_PLUGINS: fn() -> Vec<PluginFunctions> = || -> Vec<PluginFunctions> {
    let mut plugins = Vec::new();
    unsafe {
        for lib in LIBS.iter() {
            let dbus_interface: Result<
                libloading::Symbol<unsafe extern "C" fn(&mut Crossroads)>, // -> Plugin>,
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
            let capabilities: Result<
                libloading::Symbol<unsafe extern "C" fn() -> PluginCapabilities>,
                libloading::Error,
            > = lib.get(b"capabilities");
            let name: Result<
                libloading::Symbol<unsafe extern "C" fn() -> String>,
                libloading::Error,
            > = lib.get(b"name");
            let tests: Result<
                libloading::Symbol<unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
                libloading::Error,
            > = lib.get(b"backend_tests");
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
                Ok(dbus_interface),
                Ok(startup),
                Ok(shutdown),
                Ok(capabilities),
                Ok(name),
                Ok(tests),
                Ok(startup_frontend),
                Ok(shutdown_frontend),
                Ok(data_frontend),
                Ok(tests_frontend),
            ) = (dbus_interface, startup, shutdown, capabilities, name, tests, startup_frontend, shutdown_frontend, data_frontend, tests_frontend)
            {
                plugins.push(PluginFunctions::new(
                    startup,
                    shutdown,
                    capabilities,
                    name,
                    dbus_interface,
                    tests,
                    startup_frontend,
                    shutdown_frontend,
                    data_frontend,
                    tests_frontend,
                ));
            } else {
                ERROR!("Failed to load plugin", ErrorLevel::Critical);
            }
        }
    }
    plugins
};

#[allow(improper_ctypes_definitions)]
pub struct PluginFunctions {
    pub startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub capabilities: libloading::Symbol<'static, unsafe extern "C" fn() -> PluginCapabilities>,
    pub name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
    pub data: libloading::Symbol<'static, unsafe extern "C" fn(&mut Crossroads)>, //-> Plugin>,
    pub tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,

    pub frontend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub frontend_data: libloading::Symbol<'static, unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>)>,
    pub frontend_tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
}

#[allow(improper_ctypes_definitions)]
impl PluginFunctions {
    pub fn new(
        backend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
        shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
        capabilities: libloading::Symbol<'static, unsafe extern "C" fn() -> PluginCapabilities>,
        name: libloading::Symbol<'static, unsafe extern "C" fn() -> String>,
        data: libloading::Symbol<'static, unsafe extern "C" fn(&mut Crossroads)>,
        tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
        frontend_startup: libloading::Symbol<'static, unsafe extern "C" fn()>,
        frontend_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
        frontend_data: libloading::Symbol<'static, unsafe extern "C" fn() -> (SidebarInfo, Vec<gtk::Box>)>,
        frontend_tests: libloading::Symbol<'static, unsafe extern "C" fn() -> Vec<PluginTestFunc>>,
    ) -> Self {
        Self {
            startup: backend_startup,
            shutdown,
            capabilities,
            name,
            data,
            tests,
            frontend_startup,
            frontend_shutdown,
            frontend_data,
            frontend_tests,
        }
    }
}

unsafe impl Send for PluginFunctions {}

unsafe impl Sync for PluginFunctions {}
