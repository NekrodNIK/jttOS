#![no_std]
#![no_main]
#![feature(maybe_uninit_write_slice)]
extern crate alloc;

mod allocator;
mod console;
mod idt;
mod io;
mod port;
mod sync;
mod utils;

use core::arch::asm;
use core::panic::PanicInfo;
use idt::Idt;
use io::Write;
use utils::EFlags;
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
        EFlags::new().write();
    }

    macro_rules! set_regs {
        () => {
            concat!(
                "mov eax, 1984\n",
                "mov ecx, 0xbebebebe\n",
                "mov edx, 0x1badb002\n",
                "mov ebx, 0\n",
                "mov ebp, 0xbeef\n",
                "mov esi, 0xCb\n",
                "mov edi, 0xdd\n",
            )
        };
    }

    #[cfg(e1)]
    unsafe {
        asm!(set_regs!(), "div ebx")
    }

    #[cfg(e2)]
    unsafe {
        asm!(set_regs!(), "int 0x13")
    }

    #[cfg(e3)]
    unsafe {
        asm!(set_regs!(), "sti")
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
