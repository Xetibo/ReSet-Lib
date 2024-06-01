#![feature(trait_upcasting)]
#![feature(string_remove_matches)]
#![feature(unsized_fn_params)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use std::{fmt, fs, iter::Peekable, path::PathBuf, slice::Iter};
use utils::{
    flags::{Flag, Flags},
    variant::{Empty, TVariant},
};

#[cfg(debug_assertions)]
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

pub fn create_config_directory(project_name: &str) -> Option<PathBuf> {
    let base_dir = xdg::BaseDirectories::new();
    if base_dir.is_err() {
        ERROR!("Could not get base directories", ErrorLevel::Critical);
        return None;
    }
    let base_dir = base_dir.unwrap();
    let config_dir = base_dir.create_config_directory(project_name);
    if config_dir.is_err() {
        ERROR!("Could not create config directory", ErrorLevel::Critical);
        return None;
    }
    Some(config_dir.unwrap())
}

pub fn create_config(project_name: &str) -> Option<PathBuf> {
    create_config_directory(project_name);
    let base_dir = xdg::BaseDirectories::new();
    if base_dir.is_err() {
        ERROR!("Could not get base directories", ErrorLevel::Critical);
        return None;
    }
    let base_dir = base_dir.unwrap();
    let mut config_file = base_dir.find_config_file(format!("{}/ReSet.toml", project_name));
    if config_file.is_none() {
        let res = base_dir.place_config_file(format!("{}/ReSet.toml", project_name));
        if let Err(error) = res {
            ERROR!(
                format!("Could not create config file: {}", error),
                ErrorLevel::Critical
            );
            return None;
        }
        config_file = Some(res.unwrap());
    }
    let config_file = config_file.unwrap();
    flatpak_fix(config_file)
}

// Hacky flatpak workaround
pub fn flatpak_fix(path_buf: PathBuf) -> Option<PathBuf> {
    let hacked_path = path_buf.to_str().unwrap().to_string();
    if hacked_path.contains("var/app") {
        Some(PathBuf::from(hacked_path.replace(
            "var/app/org.Xetibo.ReSet/config/ReSet.toml",
            "config/reset/ReSet.toml",
        )))
    } else {
        Some(path_buf)
    }
}

pub fn parse_flags(flags: Vec<String>) -> Flags {
    let mut parsed_flags = Flags(Vec::new());
    let mut iter = flags.iter().peekable();
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

fn handle_config(flags: &mut Flags, file: Option<&String>) {
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
    flags.0.push(Flag::ConfigDir(path.clone()));
}

fn handle_plugins(flags: &mut Flags, file: Option<&String>) {
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
    flags.0.push(Flag::PluginDir(path.clone()));
}

fn handle_other(flags: &mut Flags, flag: &str, values: &mut Peekable<Iter<String>>) {
    if !is_flag(flag) {
        ERROR!(
            format!("Expected a flag, got a regular string instead: {}", flag),
            ErrorLevel::Critical
        );
        return;
    }
    let mut parsed_flags = Vec::new();
    loop {
        let next = values.peek();
        if let Some(value) = next {
            if is_flag(value) {
                break;
            }
        } else {
            break;
        }
        let next = values.next();
        parsed_flags.push(next.unwrap().clone());
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

fn is_flag(maybe_flag: &str) -> bool {
    if maybe_flag.starts_with('-') && !maybe_flag.starts_with("--") && maybe_flag.len() > 1
        || maybe_flag.starts_with("--") && maybe_flag.len() > 2
    {
        return true;
    }
    false
}
