use std::fmt;

pub type ReSetError = Box<dyn TReSetError>;

pub trait TReSetError: fmt::Debug + fmt::Display + Send + Sync + 'static {}

pub fn create_error(err: impl TReSetError) -> ReSetError {
    Box::new(err)
}

impl TReSetError for zbus::Error {}
impl From<zbus::Error> for ReSetError {
    fn from(value: zbus::Error) -> Self {
        create_error(value)
    }
}

impl TReSetError for String {}
impl From<String> for ReSetError {
    fn from(value: String) -> Self {
        create_error(value)
    }
}
