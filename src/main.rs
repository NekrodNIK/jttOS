#![no_std]
#![no_main]

mod irq;
mod logs;
mod utils;
mod vga;

use core::cell::LazyCell;
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::irq::IrqSafe;
use crate::vga::Vga;

const LOGO: &'static str = include_str!("logo.txt");

static VGA: IrqSafe<LazyCell<Vga>> = IrqSafe::new(LazyCell::new(|| Vga::new()));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = VGA.lock();
    vga.clear();
    write!(vga, "{}\n\n", LOGO).unwrap();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VGA.lock();
    vga.clear();
    write!(vga, "panic: {}", info.message()).unwrap();
    loop {}
}
