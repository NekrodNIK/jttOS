#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod critical_section;
mod device_manager;
mod drivers;
mod entry;
mod gdt;
mod interrupts;
mod panic;
mod syscalls;
mod tss;
mod x86_utils;

use alloc::boxed::Box;
use core::{
    mem,
    sync::atomic::{AtomicBool, Ordering},
};
use device_manager::DEVICES;
use utils::io::Write;

use crate::{
    gdt::GDT,
    interrupts::Idt,
    tss::TSS,
    x86_utils::{EFlags, tsc_sleep},
};
use core::arch::asm;

#[macro_export]
macro_rules! info {
    ($tbw:expr, $($arg:tt)*) => {{
        write!($tbw, "[").unwrap();
        $tbw.set_next_fg(0x0000ff00);
        write!($tbw, "INFO").unwrap();
        $tbw.set_next_fg(0x00ffffff);
        write!($tbw, "] {}\n", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! warning {
    ($tbw:expr, $($arg:tt)*) => {{
        write!($tbw, "[").unwrap();
        $tbw.set_next_fg(0x00ffff00);
        write!($tbw, "WARNING").unwrap();
        $tbw.set_next_fg(0x00ffffff);
        write!($tbw, "] {}\n", format_args!($($arg)*)).unwrap();
    }};
}

unsafe extern "C" {
    static framebuffer_addr: *mut u32;
    static framebuffer_width: u16;
    static framebuffer_height: u16;
}

fn new_tbw() -> utils::textbuffer::TextBufferWritter {
    let tbw = unsafe {
        utils::textbuffer::TextBufferWritter::new(utils::textbuffer::TextBuffer::new(
            utils::framebuffer::Framebuffer {
                addr: framebuffer_addr,
                width: framebuffer_width as usize,
                height: framebuffer_height as usize,
            },
        ))
    };
    tbw
}

pub fn kmain() {
    let mut tbw = new_tbw();
    tbw.clear();

    let mut idt = Idt::new();
    info!(tbw, "IDT loaded");
    idt.mark_syscall(0x30);
    idt.load();
    GDT.tss.update(|mut tss_desc| {
        tss_desc.set_base(&raw const TSS as u32);
        tss_desc.set_limit((mem::size_of_val(&TSS) - 1) as u32);
        tss_desc
    });

    TSS.load();
    info!(tbw, "TSS loaded");

    DEVICES.init_devices(&mut tbw);

    interrupts::register_handler(0x80, syscalls::generic_handler);
    idt.mark_syscall(0x80);
    unsafe { x86_utils::sti() };
    info!(tbw, "Interrupts enabled");

    writeln!(tbw, include_str!("logo.txt")).unwrap();
    writeln!(tbw, "Press any key...").unwrap();

    while DEVICES.ps2keyboard.read() == 0 {
        tsc_sleep(100)
    }

    jump_to_userspace(userspace::entry);
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
