use crate::parse_flags;

use super::variant::Variant;
use once_cell::sync::Lazy;

// Handles all command line flags
#[derive(Debug)]
pub enum Flag {
    ConfigDir(String),
    PluginDir(String),
    Other((String, Variant)),
}

#[allow(clippy::declare_interior_mutable_const)]
pub const FLAGS: Lazy<Flags> = Lazy::new(|| {
    let flags = std::env::args().collect::<Vec<String>>();
    parse_flags(flags)
});

#[derive(Debug)]
pub struct Flags(pub Vec<Flag>);
