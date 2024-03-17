use std::collections::HashMap;

use dbus::Path;

use super::variant::Variant;

pub fn plugin_data() -> PluginData {
    PluginData(HashMap::new())
}

#[repr(C)]
pub struct PluginCapabilities(Vec<String>);

impl PluginCapabilities {
    pub fn get_capabilities(&self) -> Vec<String> {
        self.0.clone()
    }

    pub fn new(capabilities: Vec<String>) -> Self {
        Self(capabilities)
    }
}

#[repr(C)]
pub struct PluginData(HashMap<String, Variant>);

impl PluginData {
    pub fn get_data(&self) -> HashMap<String, Variant> {
        self.0.clone()
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

