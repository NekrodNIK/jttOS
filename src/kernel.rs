#![no_std]
#![no_main]
#![feature(allocator_api)]

extern crate alloc;

mod allocator;
mod console;
mod device_manager;
mod drivers;
mod entry;
mod interrupts;
mod io;
mod lab5;
mod panic;
mod utils;

pub fn kmain() {
    console::clear!();
    lab5::run();
}
