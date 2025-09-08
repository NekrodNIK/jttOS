#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod cpuid;
mod idt;
mod log;
mod ports;
mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
use cpuid::Cpuid;
use uart::Uart;

use crate::idt::Idt;

const LOGO: &'static str = include_str!("logo.txt");

// TODO: remove 'unwrap's -> device manager
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut uart = Uart::new(0x3f8);
    unsafe { uart.init() }

    write!(uart, "{}\n\n", LOGO).unwrap();
    log::info!(uart, "kernel loaded with multiboot2 protocol").unwrap();

    let idt = Idt::new();
    unsafe { idt.load() }

    log::info!(uart, "idt loaded").unwrap();

    let mut cpu_id = Cpuid::default(); // TODO: convert single object to device manager

    log::info!(uart, "VendorID: {}", cpu_id.get_vendor()).unwrap();

    panic!("Nothing further");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: remove creation of new uart on the same port
    let mut uart = Uart::new(0x3f8);

    let _ = match info.message().as_str() {
        Some(message) => log::panic!(uart, "{}", message),
        None => log::panic!(uart, "(No message)"),
    };

    loop {}
}
