

#[macro_export]
macro_rules! dbgprintln {
    ($($t:tt)*) => {
        // if cfg!(feature = "debug") {
            // println!($($t)*);
        // }
    }
}

#[macro_export]
macro_rules! dbgprintln2 {
    ($($t:tt)*) => {
        // if cfg!(feature = "debug") {
            println!($($t)*);
        // }
    }
}