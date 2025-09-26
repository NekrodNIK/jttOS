#![no_std]
#![no_main]

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::vga::Vga;

const LOGO: &'static str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = Vga::new();
    vga.clear();
    write!(vga, "{}\n\n", LOGO).unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
