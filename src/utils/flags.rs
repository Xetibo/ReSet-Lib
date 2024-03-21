// Handles all command line flags
#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum Flag<'a> {
    ConfigDir(&'a String),
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct Flags<'a>(pub Vec<Flag<'a>>);
