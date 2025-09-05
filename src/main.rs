#![no_std]
#![no_main]

mod idt;
mod log;
mod ports;
mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
use uart::Uart;

use crate::idt::Idt;

const LOGO: &'static str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut uart = Uart::new(0x3f8);
    unsafe { uart.init() }

    write!(uart, "{}\n\n", LOGO).unwrap();
    log::info!(uart, "kernel loaded with multiboot2 protocol").unwrap();

    let idt = Idt::new();
    unsafe { idt.load() }

    log::info!(uart, "idt loaded").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
