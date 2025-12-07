#![allow(unused_imports)]

#[macro_export]
macro_rules! green {
    ($text:expr) => {
        concat!("\x1b[32m", $text, "\x1b[39m")
    };
}

#[macro_export]
macro_rules! red {
    ($text:expr) => {
        concat!("\x1b[31m", $text, "\x1b[39m")
    };
}

#[macro_export]
macro_rules! yellow {
    ($text:expr) => {
        concat!("\x1b[33m", $text, "\x1b[39m")
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::console::print!("[{}] {}\n", $crate::console::green!("INFO"), format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        $crate::console::print!("[{}] {}\n", $crate::console::yellow!("WARNING"), format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{ $crate::console::print!("{}\n", format_args!($($arg)*)) }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        unsafe {write!(*$crate::console::CONSOLE.get(), $($arg)*).unwrap()}
    }};
}

#[macro_export]
macro_rules! clear {
    ($($arg:tt)*) => {{ unsafe { (*$crate::console::CONSOLE.get()).clear() } }};
}

pub use green;
pub use red;
pub use yellow;

pub use info;
pub use warning;

pub use clear;
pub use print;
pub use println;
