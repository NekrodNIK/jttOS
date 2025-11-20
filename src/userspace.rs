// FIXME: please refactor
use crate::{
    io::Write,
    lab6,
    utils::{cli, esp},
};
use core::arch;

use crate::{console, utils::EFlags};
use alloc::boxed::Box;

unsafe extern "C" {
    static mut tss_desc: u64;
}

#[repr(C, packed)]
struct Tss {
    _reserved1: u32,
    kernel_sp: u32,
    kernel_ss: u16,
}

impl Tss {
    // TODO: safe or unsafe?
    pub unsafe fn load(&self) {
        let base = self as *const Tss as u64;
        let limit = 0x67;

        unsafe {
            tss_desc |= limit & 0xffff;
            tss_desc |= (base & 0xffff) << 16;
            tss_desc |= (base >> 16 & 0xff) << 32;
            tss_desc |= (limit >> 16 & 0xf) << 48;
            tss_desc |= (base >> 24 & 0xff) << 56;
            arch::asm!("ltr {0:x}", in(reg) 40);
        }
    }
}

pub fn run(f: fn()) {
    let tss = Box::new(Tss {
        _reserved1: 0,
        kernel_sp: 0x7c00,
        kernel_ss: 16,
    });

    unsafe {
        tss.load();
        Box::leak(tss);

        let stack = Box::into_raw(Box::new([0usize; 4 * 1024]));
        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);
        arch::asm!(
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
            eip = in(reg) f,
            ds = in(reg) 32 | 0b11,
            options(nostack),
        );
    }
}
