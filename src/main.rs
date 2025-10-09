#![no_std]
#![no_main]

mod ansi_escape;
mod console;
mod io;
mod irq;
mod logs;
mod utils;

use core::cell::LazyCell;
use core::panic::PanicInfo;

use crate::console::Console;
use crate::io::Write;
use crate::irq::IrqSafe;

const LOGO: &'static str = include_str!("logo.txt");

static CONSOLE: IrqSafe<LazyCell<Console>> = IrqSafe::new(LazyCell::new(|| Console::new()));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut console = CONSOLE.lock();
    console.clear();
    write!(console, "{}\n", LOGO).unwrap();
    logs::info!(console, "{}", "Loading system...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut console = CONSOLE.lock();
    logs::panic!(console, "{}", info.message());
    loop {}
}
