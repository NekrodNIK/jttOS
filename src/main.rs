#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod device;
mod interrupts;
mod io;
mod pic8259;
mod port;
mod ps2;
mod ringbuffer;
mod sync;
mod utils;

use core::cell::LazyCell;
use core::panic::PanicInfo;

use alloc::boxed::Box;
use core::arch::asm;

use crate::interrupts::{Idt, InterruptContext};
use crate::io::Write;
use crate::pic8259::ChainedPics;
use crate::ps2::Keyboard;
use crate::sync::IntSafe;
use crate::utils::{EFlags, cli, sti};

const LOGO: &str = include_str!("logo.txt");

static PICS: IntSafe<ChainedPics> = IntSafe::new(ChainedPics::new(0x20, 0x28));
static KEYBOARD: IntSafe<Keyboard> = IntSafe::new(Keyboard::new());

unsafe extern "C" {
    pub fn e1();
    pub fn e2();
    pub fn e3();
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);

    let idt = Box::new(Idt::new());
    idt.load();
    console::info!("{}", "IDT loaded");

    PICS.lock().init();
    console::info!("{}", "PIC initialized");

    KEYBOARD.lock().init();
    interrupts::register_handler(0x21, |ctx: &InterruptContext| {
        KEYBOARD.lock().int_handler(ctx)
    });
    PICS.lock().enable_device(1);
    console::info!("{}", "PS/2 keyboard initialized");

    unsafe { sti() }
    console::info!("{}", "Interrupts enabled");

    loop {
        let mut events = ps2::KEY_EVENTS.lock();
        if events.available() {
            console::info!("{:?}", events.pop_front().unwrap());
        }
        utils::tsc_sleep(1000000);
    }
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
