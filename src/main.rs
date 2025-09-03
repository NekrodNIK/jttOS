#![no_std]
#![no_main]

mod ports;
mod uart;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use uart::Uart;
use vga::Vga;

const LOGO: &'static str = r"   _ _   _    ___  ____
  (_) |_| |_ / _ \/ ___| 
  | | __| __| | | \___ \ 
  | | |_| |_| |_| |___) |
  / |\__|\__|\___/|____/ 
|__/
";

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = Vga::new();
    vga.clear();

    write!(vga, "{}", LOGO).unwrap();

    let mut uart = Uart::new(0x3f8);
    unsafe {
        uart.init();
    }

    write!(uart, "{}", "\x1b[31mHello\n").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
