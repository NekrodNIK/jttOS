#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod idt;
mod io;
mod pic8259;
mod port;
mod sync;
mod utils;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use idt::Idt;
use io::Write;
use utils::EFlags;
use utils::cli;

use crate::pic8259::ChainedPics;

const LOGO: &str = include_str!("logo.txt");

unsafe extern "C" {
    pub fn e1();
    pub fn e2();
    pub fn e3();
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);
    console::info!("{}", "Loading system...");

    let idt = Box::new(Idt::new());
    idt.load();

    let pics = ChainedPics::new(0x20, 0x28);
    pics.init();

    unsafe {
        (EFlags::new() | EFlags::IF).write();
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }
    console::CONSOLE.try_unlock();
    console::clear!();
    console::println!("[{}]", console::red!("KERNEL PANIC"));
    console::print!("{}", info.message());
    loop {}
}
