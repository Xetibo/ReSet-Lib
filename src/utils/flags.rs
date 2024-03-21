use super::variant::Variant;

// Handles all command line flags
#[derive(Debug)]
pub enum Flag<'a> {
    ConfigDir(&'a String),
    PluginDir(&'a String),
    Other((String,Variant))
}

#[derive(Debug)]
pub struct Flags<'a>(pub Vec<Flag<'a>>);
