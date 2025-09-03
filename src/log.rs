use core::format_args;

#[macro_export]
macro_rules! green {
    ($text:expr) => {
        concat!("\x1b[32m", $text, "\x1b[0m")
    };
}

#[macro_export]
macro_rules! info {
    ($dst:expr, $($arg:tt)*) => {{
        write!($dst, "[{}] {}\n", green!("INFO"), format_args!($($arg)*));
    }};
}

pub use info;
