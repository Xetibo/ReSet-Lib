use std::{fs, io::Read};

use once_cell::sync::Lazy;
use toml::Table;

use crate::{create_config, ERROR, LOG};
#[cfg(debug_assertions)]
use crate::{utils::macros::ErrorLevel, write_log_to_file};

pub static mut CONFIG_STRING: Lazy<String> = Lazy::new(|| {
    let config = create_config("reset");
    if let Some(config) = config {
        config.to_str().unwrap().to_string()
    } else {
        ERROR!("Failed to get user home", ErrorLevel::Critical);
        String::from("")
    }
});
pub static CONFIG: Lazy<Table> = Lazy::new(parse_config);

pub fn parse_config() -> Table {
    unsafe {
        let config_file = fs::File::open(CONFIG_STRING.as_str());
        LOG!(format!(
            "Using config file path: {}",
            CONFIG_STRING.as_str()
        ));
        if let Err(_errorr) = config_file {
            ERROR!(
                format!("Could not write config file: {}", _errorr),
                ErrorLevel::Recoverable
            );
            return Table::new();
        }
        let mut config_string = String::from("");
        let err = config_file.unwrap().read_to_string(&mut config_string);
        if let Err(_error) = err {
            ERROR!(
                format!("Could not read config file: {}", _error),
                ErrorLevel::Recoverable
            );
            return Table::new();
        }
        LOG!(format!("Config file content:\n {}", config_string));
        config_string.parse::<Table>().expect("Config has errors")
    }
}

pub fn get_config_value<T, F: Fn(&toml::value::Value) -> T>(
    category: &'static str,
    entry: &'static str,
    callback: F,
) -> bool {
    #[allow(clippy::borrow_interior_mutable_const)]
    if let Some(monitor_config) = CONFIG.get(category) {
        if let Some(value) = monitor_config.get(entry) {
            (callback(value));
            return true;
        }
    }
    false
}
