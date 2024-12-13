

use std::sync::atomic::{AtomicUsize, Ordering};

/// Debug log levels
#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// Global log level, defaulting to Info
static GLOBAL_LOG_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::Info as usize);

/// Set the global log level
pub fn set_log_level(level: LogLevel) {
    GLOBAL_LOG_LEVEL.store(level as usize, Ordering::Relaxed);
}

/// Macro for conditional logging
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let current_level = $crate::utils::GLOBAL_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        let target_level = $level as usize;
        if current_level >= target_level {
            println!("[{:?}] {}", $level, format!($($arg)*));
        }
    }};
}

/// Convenience logging macros
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

// Backwards compatibility macros
#[macro_export]
macro_rules! dbgprintln {
    ($($arg:tt)*) => { log_debug!($($arg)*) }
}

#[macro_export]
macro_rules! dbgprintln2 {
    ($($arg:tt)*) => { log_trace!($($arg)*) }
}
