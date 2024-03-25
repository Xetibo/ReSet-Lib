#![feature(trait_upcasting)]
#![feature(unsized_fn_params)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use directories_next as dirs;
use std::{fmt, fs, path::PathBuf, slice::Iter};
use utils::{
    flags::{Flag, Flags},
    variant::{Empty, TVariant},
};

use crate::utils::macros::ErrorLevel;

pub mod audio;
pub mod bluetooth;
pub mod network;
pub mod signals;
mod tests;
pub mod utils;

#[derive(Debug, Clone)]
pub struct PathNotFoundError;

/// Version of the current package.
/// Use this to avoid version mismatch conflicts.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

impl fmt::Display for PathNotFoundError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "File could not be found")
    }
}

pub fn create_config(project_organization: &str, project_name: &str) -> Option<PathBuf> {
    let config_dir = dirs::ProjectDirs::from("org", project_organization, project_name)?;
    let config_dir = config_dir.config_dir();
    if !config_dir.exists() {
        fs::create_dir(config_dir).expect("Could not create directory");
    }
    let metadata = fs::metadata(config_dir);
    if metadata.is_err() {
        return None;
    }
    let config_file = String::from(project_name) + ".toml";
    let file_path = config_dir.join(config_file);
    if !file_path.exists() {
        fs::File::create(&file_path).expect("Could not write config file");
    }
    Some(config_dir.join(""))
}

pub fn parse_flags(flags: &[String]) -> Flags {
    let mut parsed_flags = Flags(Vec::new());
    let mut iter = flags.iter();
    iter.next().expect("Did not recieve a binary name!");
    loop {
        let next = iter.next();
        if next.is_none() {
            break;
        }
        let flag = next.unwrap().as_str();
        match flag {
            "--config" => handle_config(&mut parsed_flags, iter.next()),
            "--plugins" => handle_plugins(&mut parsed_flags, iter.next()),
            _ => handle_other(&mut parsed_flags, flag, &mut iter),
        }
    }
    parsed_flags
}

fn handle_config<'a>(flags: &mut Flags<'a>, file: Option<&'a String>) {
    if file.is_none() {
        ERROR!("No file provided!", ErrorLevel::Critical);
        return;
    }
    let path = file.unwrap();
    let data = fs::metadata(path);
    if data.is_err() {
        ERROR!("Provided path does not exist!", ErrorLevel::Critical);
        return;
    }
    let data = data.unwrap();
    if !data.is_file() {
        ERROR!("Provided path is not a file!", ErrorLevel::Critical);
        return;
    }
    flags.0.push(Flag::ConfigDir(path));
}

fn handle_plugins<'a>(flags: &mut Flags<'a>, file: Option<&'a String>) {
    if file.is_none() {
        ERROR!("No directory provided!", ErrorLevel::Critical);
        return;
    }
    let path = file.unwrap();
    let data = fs::metadata(path);
    if data.is_err() {
        ERROR!("Provided path does not exist!", ErrorLevel::Critical);
        return;
    }
    let data = data.unwrap();
    if !data.is_dir() {
        ERROR!("Provided path is not a directory!", ErrorLevel::Critical);
        return;
    }
    flags.0.push(Flag::PluginDir(path));
}

fn handle_other<'a>(flags: &mut Flags<'a>, flag: &'a str, values: &mut Iter<String>) {
    if !flag.starts_with('-') || !flag.starts_with("--") {
        ERROR!(
            format!("Expected a flag, got a regular string instead: {}", flag),
            ErrorLevel::Critical
        );
        return;
    }
    if values.len() == 0 {
        return;
    }
    let mut parsed_flags = Vec::new();
    loop {
        let next = values.next();
        if let Some(value) = next {
            parsed_flags.push(value.clone());
        } else {
            break;
        }
    }
    match parsed_flags.len() {
        0 => flags
            .0
            .push(Flag::Other((flag.to_string(), Empty {}.into_variant()))),
        1 => flags.0.push(Flag::Other((
            flag.to_string(),
            parsed_flags.pop().unwrap().into_variant(),
        ))),
        _ => flags
            .0
            .push(Flag::Other((flag.to_string(), parsed_flags.into_variant()))),
    }
}
