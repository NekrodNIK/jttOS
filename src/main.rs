#![no_std]
#![no_main]
#![feature(maybe_uninit_write_slice)]
extern crate alloc;

mod allocator;
mod console;
mod idt;
mod io;
mod irq;
mod port;
mod utils;

use core::arch::asm;
use core::panic::PanicInfo;
use idt::Idt;
use io::Write;

use utils::sti;

use utils::cli;

const LOGO: &str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);
    console::info!("{}", "Loading system...");

    let idt = Idt::new();
    idt.load();
    unsafe {
        sti();
        asm!("int 0x10")
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
