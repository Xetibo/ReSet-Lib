#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! LOG {
    ($message:expr) => {{}};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! LOG {
    ($message:expr) => {{
        write_log_to_file!($message);
        println!("LOG: {}", $message);
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! ERROR {
    ($message:expr, $level:expr ) => {{}};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! ERROR {
    ($message:expr, $level:expr) => {{
        write_log_to_file!($message);
        match $level {
            ErrorLevel::Recoverable => println!("Minor Error: {}", $message),
            ErrorLevel::PartialBreakage => println!("Partial Error: {}", $message),
            ErrorLevel::Critical => println!("Critical Error: {}", $message),
        };
    }};
}
#[macro_export]
macro_rules! write_log_to_file {
    ($message:expr) => {{
        use std::{fs::OpenOptions, io::Write};
        const VERSION: &str = env!("CARGO_PKG_NAME");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("/tmp/".to_string() + VERSION + "_log")
            .expect("Could not open log file");
        file.write_all($message.as_bytes())
            .expect("Could not write to log file");
    }};
}

pub enum ErrorLevel {
    Recoverable,
    PartialBreakage,
    Critical,
}
