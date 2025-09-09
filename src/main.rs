#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// mod cpuinfo;
mod idt;
mod log;
mod ports;
mod uart;

use core::fmt::Write;
use core::mem;
use core::panic::PanicInfo;
// use cpuinfo::CpuInfo;
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

    // let vendor = CpuInfo::read_vendor();
    // log::info!(uart, "VendorID: {}", str::from_utf8(&vendor).unwrap()).unwrap();

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
