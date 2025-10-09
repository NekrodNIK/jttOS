#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod io;
mod irq;
mod log;
mod utils;

use core::cell::LazyCell;
use core::panic::PanicInfo;

use alloc::string::{String, ToString};

use crate::console::Console;
use crate::io::Write;
use crate::irq::IrqSafe;
use crate::utils::{cli, tsc_sleep};

const LOGO: &'static str = include_str!("logo.txt");

static CONSOLE: IrqSafe<LazyCell<Console>> = IrqSafe::new(LazyCell::new(|| Console::new()));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut console = CONSOLE.lock();
    console.clear();
    write!(console, "{}\n", LOGO).unwrap();
    log::info!(console, "{}", "Loading system...").unwrap();

    // DEMO

    // let mut index = 0;
    // loop {
    //     log::info!(console, "I'm scrolling! index: {}", index).unwrap();
    //     index += 1;
    //     tsc_sleep(20000000);
    // }

    // debug_assert!("Answer to the Ultimate Question of Life, the Universe, and Everything" == "42");

    // panic!("Some panic message");

    // let mut string = "Message".to_string();
    // string += " + another message";
    // log::info!(console, "{}", string);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }
    while let Err(_) = panic_message(info) {}
    loop {}
}

fn panic_message(info: &PanicInfo) -> io::Result<()> {
    let mut console = CONSOLE.lock();
    console.clear();
    write!(console, "[{}]\n", red!("KERNEL PANIC"))?;
    write!(console, "{}", info.message())?;
    Ok(())
}
