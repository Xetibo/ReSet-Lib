use std::{fs, io::Read};

use once_cell::sync::Lazy;
use toml::Table;

use crate::{utils::macros::ErrorLevel, write_log_to_file, ERROR};

pub static mut CONFIG_STRING: &str = "~/reset/ReSet.toml";
#[allow(clippy::declare_interior_mutable_const)]
pub const CONFIG: Lazy<Table> = Lazy::new(parse_config);

pub fn parse_config() -> Table {
    unsafe {
        let config_file = fs::File::open(CONFIG_STRING);
        if config_file.is_err() {
            ERROR!("Could not write config file", ErrorLevel::Recoverable);
            return Table::new();
        }
        let mut config_string = String::from("");
        let err = config_file
            .unwrap()
            .read_to_string(&mut config_string)
            .is_err();
        if err {
            ERROR!("Could not read config file", ErrorLevel::Recoverable);
            return Table::new();
        }
        config_string.parse::<Table>().expect("Config has errors")
    }
}
