#![no_std]
#![no_main]
#![feature(allocator_api)]

use core::arch;

use crate::{device_manager::DEVICES, interrupts::Idt};

extern crate alloc;

mod allocator;
mod console;
mod critical_section;
mod device_manager;
mod drivers;
mod entry;
mod interrupts;
mod io;
mod lab6;
mod panic;
mod userspace;
mod utils;

pub fn kmain() {
    console::clear!();
    let mut idt = Idt::new();
    idt.load();
    DEVICES.init_devices();
    lab6::run();
}
