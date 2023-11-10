use directories_next as dirs;
use std::{fmt, fs, path::PathBuf};

mod tests;
pub mod audio;
pub mod network;
pub mod bluetooth;
pub mod signals;
pub mod utils;

#[derive(Debug, Clone)]
struct PathNotFoundError;

impl fmt::Display for PathNotFoundError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "File could not be found")
    }
}

#[allow(dead_code)]
fn create_config(project_organization: &str, project_name: &str) -> Option<PathBuf> {
    let config_dir = dirs::ProjectDirs::from("com", project_organization, project_name)?;
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
