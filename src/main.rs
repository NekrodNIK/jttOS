#![no_std]
#![no_main]

mod log;
mod ports;
mod uart;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use uart::Uart;
use vga::Vga;

const LOGO: &'static str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut uart = Uart::new(0x3f8);
    unsafe {
        uart.init();
    }

    write!(uart, "{}\n\n", LOGO);
    log::info!(uart, "the kernel was loaded by the bootloader");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
