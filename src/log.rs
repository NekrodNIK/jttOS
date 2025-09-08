use core::format_args;

// TODO: simplify and add setLogger

#[macro_export]
macro_rules! green {
    ($text:expr) => {
        concat!("\x1b[32m", $text, "\x1b[0m")
    };
}

#[macro_export]
macro_rules! red {
    ($text:expr) => {
        concat!("\x1b[31m", $text, "\x1b[0m")
    };
}

macro_rules! info {
    ($dst:expr, $($arg:tt)*) => {{
        write!($dst, "[{}] {}\n", green!("INFO"), format_args!($($arg)*))
    }};
}

macro_rules! panic {
    ($dst:expr, $($arg:tt)*) => {{
        write!($dst, "[{}] {}\n", red!("PANIC"), format_args!($($arg)*))
    }};
}

pub(crate) use info;
pub(crate) use panic;
