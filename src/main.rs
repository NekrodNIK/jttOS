#![no_std]
#![no_main]
#![feature(maybe_uninit_write_slice)]
extern crate alloc;

mod allocator;
mod console;
mod io;
mod irq;
mod port;
mod sync;
mod utils;

use core::arch::asm;
use core::panic::PanicInfo;
use io::Write;
use sync::Idt;

use utils::sti;

use utils::cli;

use crate::utils::EFlags;

const LOGO: &str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);
    console::info!("{}", "Loading system...");

    let idt = Idt::new();
    idt.load();

    #[cfg(e1)]
    unsafe {
        asm!("int 0x11")
    }

    #[cfg(e2)]
    {
        unsafe { asm!("int 0x15") }
    }

    #[cfg(e3)]
    {
        unsafe { asm!("int 0x15") }
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
