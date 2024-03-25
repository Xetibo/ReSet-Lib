use std::{collections::HashMap, error::Error, fmt::Display};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use dbus::Path;

use super::variant::Variant;

pub fn plugin_data() -> PluginData {
    PluginData(HashMap::new())
}

#[repr(C)]
pub struct PluginCapabilities(Vec<&'static str>);

impl PluginCapabilities {
    pub fn get_capabilities(&self) -> Vec<&'static str> {
        self.0.clone()
    }

    pub fn new(capabilities: Vec<&'static str>) -> Self {
        Self(capabilities)
    }
}

#[repr(C)]
pub struct PluginData(HashMap<String, Variant>);

impl PluginData {
    pub fn get_data(&self) -> HashMap<String, Variant> {
        self.0.clone()
    }

    pub fn move_data(self) -> HashMap<String, Variant> {
        self.0
    }

    pub fn get_data_ref(&self) -> &HashMap<String, Variant> {
        &self.0
    }

    pub fn new(map: HashMap<String, Variant>) -> Self {
        Self(map)
    }
}

#[repr(C)]
pub struct Plugin {
    pub path: Path<'static>,
    pub interfaces: Vec<dbus_crossroads::IfaceToken<PluginData>>,
    pub data: PluginData,
}

impl Plugin {
    pub fn new(
        path: Path<'static>,
        interfaces: Vec<dbus_crossroads::IfaceToken<PluginData>>,
        data: PluginData,
    ) -> Self {
        Self {
            path,
            interfaces,
            data,
        }
    }
}

#[derive(Debug)]
pub struct PluginTestError(&'static str);

impl PluginTestError {
    pub fn new(message: &'static str) -> Self {
        Self(message)
    }

    pub fn message(&self) -> &'static str {
        self.0
    }
}

impl Display for PluginTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for PluginTestError {}

unsafe impl Send for PluginTestError {}
unsafe impl Sync for PluginTestError {}

pub struct PluginTestFunc(
    Box<dyn FnOnce() -> Result<(), PluginTestError>>,
    &'static str,
);

impl PluginTestFunc {
    pub fn new(
        func: impl Fn() -> Result<(), PluginTestError> + 'static,
        name: &'static str,
    ) -> Self {
        Self(Box::new(func), name)
    }

    pub fn name(&self) -> &'static str {
        self.1
    }
}

impl FnOnce<()> for PluginTestFunc {
    type Output = Result<(), PluginTestError>;

    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.0.call_once(())
    }
}

unsafe impl Send for PluginTestFunc {}
unsafe impl Sync for PluginTestFunc {}

pub fn plugin_tests(plugin_name: impl AsRef<str>, tests: Vec<PluginTestFunc>) {
    use std::thread;

    let mut running = String::from("");
    let running_index = tests.len();
    let mut crashed = String::from("");
    let mut crashed_index = 0;
    let mut failed = String::from("");
    let mut failed_index = 0;
    let mut success = String::from("");
    let mut success_index = 0;
    for func in tests {
        let name = func.name();
        running += &format!("running test {}\n", name);
        let test = thread::spawn(func).join();
        if test.is_err() {
            // panic is currently handled differently
            crashed += &format!("Thread of test {} crashed!\n", name);
            crashed_index += 1;
        } else if let Ok(Err(error)) = test {
            failed += &format!("Test {} failed with error: {}\n", name, error.message());
            failed_index += 1;
        } else {
            success += &format!("Success: {}\n", name);
            success_index += 1;
        }
    }
    let mut buffer = String::from("");
    buffer += &format!(
        "\n----------- Plugin Tests for {} -----------\n\n",
        plugin_name.as_ref()
    );
    buffer += &format!("running {} tests:\n", running_index);
    buffer += &running;
    buffer += &format!("\n{} test crashed:\n", crashed_index);
    buffer += &crashed;
    buffer += &format!("\n{} tests failed:\n", failed_index);
    buffer += &failed;
    buffer += &format!("\n{} tests successful:\n", success_index);
    buffer += &success;
    buffer += "\n----------- Plugin Tests end -----------\n\n";
    print!("{}", buffer);
    // this combination is done to avoid conflicts with other tests
    // -> the cli only has one buffer, e.g. multiple threads writing
    // to it could cause conflicts with the terminal output
}

#[repr(C)]
pub struct SidebarInfo {
    pub name: &'static str,
    pub icon_name: &'static str,
    pub parent: Option<ParentInfo>,
}

#[repr(C)]
pub struct ParentInfo {
    pub name: &'static str,
    pub children: i32,
}
