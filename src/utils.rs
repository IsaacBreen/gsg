

#[macro_export]
macro_rules! debug {
    // Allow just a message without level (defaults to level 1)
    ($($arg:tt)*) => {
        $crate::debug!(1, $($arg)*);
    };

    // Allow level + message
    ($level:expr, $($arg:tt)*) => {{
        if $level <= $crate::DEBUG_LEVEL {
            #[cfg(feature = "debug")]
            println!("[DEBUG {}] {}", $level, format!($($arg)*));
        }
    }};
}
