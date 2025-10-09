#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod io;
mod irq;
mod utils;

use crate::io::Write;
use crate::utils::{cli, tsc_sleep};
use core::panic::PanicInfo;

const LOGO: &'static str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);
    console::info!("{}", "Loading system...");

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
    console::CONSOLE.try_unlock();
    console::clear!();
    console::println!("[{}]", red!("KERNEL PANIC"));
    console::print!("{}", info.message());
    loop {}
}
