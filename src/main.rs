#![no_std]
#![no_main]

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
use crate::utils::cli;

const LOGO: &'static str = include_str!("logo.txt");

static CONSOLE: IrqSafe<LazyCell<Console>> = IrqSafe::new(LazyCell::new(|| Console::new()));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut console = CONSOLE.lock();
    console.clear();
    write!(console, "{}\n", LOGO).unwrap();
    logs::info!(console, "{}", "Loading system...");
    // panic!("Some panic message");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }

    let mut console = CONSOLE.lock();
    console.clear();
    write!(console, "[{}]\n", red!("KERNEL PANIC"));
    write!(console, "{}", info.message());

    loop {}
}
