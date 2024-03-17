use std::collections::HashMap;

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
}

#[repr(C)]
pub struct PluginData(HashMap<String, Variant>);

impl PluginData {
    pub fn get_data(&self) -> HashMap<String, Variant> {
        self.0.clone()
    }
}
