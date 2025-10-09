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

macro_rules! info {
    ($dst:expr, $($arg:tt)*) => {{
        write!($dst, "[{}] {}\n", green!("INFO"), format_args!($($arg)*))
    }};
}

pub(crate) use green;
pub(crate) use info;
pub(crate) use red;
