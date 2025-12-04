#![no_std]
#![no_main]
#![feature(allocator_api)]

extern crate alloc;

mod allocator;
mod console;
mod critical_section;
mod descriptors;
mod device_manager;
mod drivers;
mod entry;
mod gdt;
mod interrupts;
mod io;
mod lab6;
mod panic;
mod tss;
mod utils;

use alloc::boxed::Box;
use io::Write;

use crate::gdt::GDT;
use crate::tss::TSS;
use crate::{device_manager::DEVICES, interrupts::Idt, utils::EFlags};
use core::arch::asm;
use core::mem;

pub fn kmain() {
    console::clear!();

    let mut idt = Idt::new();
    idt.mark_syscall(0x30);
    idt.load();
    console::info!("IDT loaded");

    GDT.tss.update(|mut tss_desc| {
        tss_desc.set_base(&raw const TSS as u32);
        tss_desc.set_limit((mem::size_of_val(&TSS) - 1) as u32);
        tss_desc
    });

    TSS.load();
    console::info!("TSS loaded");
    DEVICES.init_devices();
    unsafe { utils::sti() };
    lab6::run();
}

pub fn jump_to_userspace(entry: fn()) {
    unsafe {
        let stack = Box::into_raw(Box::new([0usize; 4 * 1024]));
        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);
        asm!(
            "push {ss}",
            "push {esp}",
            "push {eflags}",
            "push {cs}",
            "push {eip}",
            "mov ds, {ds}",
            "mov es, {ds}",
            "mov fs, {ds}",
            "mov gs, {ds}",
            "iretd",
            ss = in(reg) 32 | 0b11,
            esp = in(reg) stack as u32 + 4 * 1024,
            eflags = in(reg) flags.bits(),
            cs = in(reg) 24 | 0b11,
            eip = in(reg) entry,
            ds = in(reg) 32 | 0b11,
            options(nostack),
        );
    }
}
