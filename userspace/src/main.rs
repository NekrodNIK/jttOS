#![no_std]
#![no_main]

mod stdlib;
use core::arch::asm;

pub fn main(args: &[*const u8]) {
    unsafe { asm!("2: sub esp, 4*1024", "call 2b") }

    loop {}
}
