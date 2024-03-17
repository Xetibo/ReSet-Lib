use std::collections::HashMap;

use super::variant::TVariant;

pub fn plugin_data() -> HashMap<String, Box<dyn TVariant>> {
    HashMap::new()
}
