use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    pub struct EFlags: u32 {
        const ID = 1 << 21;
        const VIP = 1 << 20;
        const VIF = 1 << 19;
        const AC = 1 << 18;
        const VM = 1 << 17;
        const RF = 1 << 16;
        const NT = 1 << 14;
        const IOPL0 = 0b00 << 12;
        const IOPL1 = 0b01 << 12;
        const IOPL2 = 0b10 << 12;
        const IOPL3 = 0b11 << 12;
        const OF = 1 << 11;
        const DF = 1 << 10;
        const IF = 1 << 9;
        const TF = 1 << 8;
        const SF = 1 << 7;
        const ZF = 1 << 6;
        const AF = 1 << 4;
        const PF = 1 << 2;
        const A1 = 1 << 1;
        const CF = 1 << 0;
    }
}

impl EFlags {
    pub fn new() -> Self {
        Self::A1
    }

    pub fn read() -> Self {
        let flags: u32;
        unsafe {
            asm!(
                "pushfd",
                "pop {}",
                out(reg) flags,
                options(nostack, preserves_flags)
            )
        }
        return Self::from_bits_retain(flags);
    }

    pub unsafe fn write(&self) {
        let flags = self.bits();
        unsafe {
            asm!(
                "push {}",
                "popfd",
                in(reg) flags,
                options(nostack, preserves_flags)
            )
        }
    }
}

pub unsafe fn cli() {
    unsafe { asm!("cli") }
}

pub unsafe fn sti() {
    unsafe { asm!("sti") }
}

pub fn rdtsc() -> u64 {
    let high: u32;
    let low: u32;

    unsafe {
        asm!("rdtsc", out("edx") high, out("eax") low);
    }
    (high as u64) << 32 | (low as u64)
}

pub fn tsc_sleep(ticks: u64) {
    let start = rdtsc();
    let mut cur = start;

    while cur - start < ticks {
        cur = rdtsc();
    }
}
