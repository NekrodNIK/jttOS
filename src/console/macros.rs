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
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::console::print!("[{}] {}\n", green!("INFO"), format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{ $crate::console::print!("{}\n", format_args!($($arg)*)) }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        write!($crate::console::CONSOLE.lock(), $($arg)*).unwrap()
    }};
}

#[macro_export]
macro_rules! clear {
    ($($arg:tt)*) => {{ $crate::console::CONSOLE.lock().clear() }};
}

pub use clear;
pub use green;
pub use info;
pub use print;
pub use println;
pub use red;
