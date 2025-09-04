#![no_std]
#![no_main]

mod idt;
mod log;
mod ports;
mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
use uart::Uart;

const LOGO: &'static str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut uart = Uart::new(0x3f8);
    unsafe {
        uart.init();
    }

    write!(uart, "{}\n\n", LOGO).unwrap();
    log::info!(uart, "kernel loaded with multiboot2 protocol").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
