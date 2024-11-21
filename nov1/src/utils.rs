

// dbgprintln! macro
#[macro_export]
macro_rules! dbgprintln {
    ($($t:tt)*) => {
        if cfg!(feature = "debug") {
            println!($($t)*);
        }
    }
}