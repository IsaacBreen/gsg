
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

static GLOBAL_LOG_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::Info as usize);

pub fn set_log_level(level: LogLevel) {
    GLOBAL_LOG_LEVEL.store(level as usize, Ordering::Relaxed);
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let current_level = $crate::utils::GLOBAL_LOG_LEVEL.load(Ordering::Relaxed);
        if current_level >= $level as usize {
            println!("[{:?}] {}", $level, format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_error { ($($arg:tt)*) => { log!(LogLevel::Error, $($arg)*) } }
#[macro_export]
macro_rules! log_warn  { ($($arg:tt)*) => { log!(LogLevel::Warn, $($arg)*) } }
#[macro_export]
macro_rules! log_info  { ($($arg:tt)*) => { log!(LogLevel::Info, $($arg)*) } }
#[macro_export]
macro_rules! log_debug { ($($arg:tt)*) => { log!(LogLevel::Debug, $($arg)*) } }
#[macro_export]
macro_rules! log_trace { ($($arg:tt)*) => { log!(LogLevel::Trace, $($arg)*) } }
