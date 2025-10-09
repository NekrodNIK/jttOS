#![no_std]
#![no_main]

mod io;
mod irq;
mod logs;
mod utils;
mod vga;

use core::cell::LazyCell;
use core::panic::PanicInfo;

use crate::io::Write;
use crate::irq::IrqSafe;
use crate::vga::Vga;

const LOGO: &'static str = include_str!("logo.txt");

static VGA: IrqSafe<LazyCell<Vga>> = IrqSafe::new(LazyCell::new(|| Vga::new()));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = VGA.lock();
    vga.clear();
    logs::info!(vga, "{}", "Loading system...");
    write!(vga, "{}\n", LOGO).unwrap();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VGA.lock();
    logs::panic!(vga, "{}", info.message());
    loop {}
}
