#[allow(unused_macros)]
macro_rules! LOG {
    ($message:expr) => {{}};
}

#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! LOG {
    ($log_file:expr, $message:expr) => {{
        write_log_to_file!($message, $log_file);
        print!("LOG: {}", $message);
    }};
}

#[allow(unused_macros)]
macro_rules! ERROR {
    ($message:expr, $level:expr ) => {{}};
}

#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! ERROR {
    ($log_file:expr, $message:expr, $level:expr) => {{
        write_log_to_file!($message, $log_file);
        match $level {
            ErrorLevel::Recoverable => print!("Minor Error: {}", $message),
            ErrorLevel::PartialBreakage => print!("Partial Error: {}", $message),
            ErrorLevel::Critical => print!("Critical Error: {}", $message),
        };
    }};
}

#[allow(unused_macros)]
macro_rules! write_log_to_file {
    ($message:expr, $log_file:expr) => {{
        use std::{fs::OpenOptions, io::Write};
        let mut file = OpenOptions::new()
            .append(true)
            .open($log_file)
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
