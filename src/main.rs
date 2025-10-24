#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod idt;
mod io;
mod port;
mod sync;
mod utils;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use idt::Idt;
use io::Write;
use utils::EFlags;
use utils::cli;

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

    unsafe {
        EFlags::new().write();
    }

    if cfg!(e1) {
        unsafe { e1() }
    }

    if cfg!(e2) {
        unsafe { e2() }
    }

    if cfg!(e3) {
        unsafe { e3() }
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
