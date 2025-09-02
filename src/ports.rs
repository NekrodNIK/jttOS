// https://wiki.osdev.org/Inline_Assembly/Examples#I.2FO_access
#![allow(dead_code)]
use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "outb dx, al",
            in("al") value,
            in("dx") port,
        )
    }
}

pub unsafe fn outw(port: u16, value: u16) {
    unsafe {
        asm!(
            "outw dx, ax",
            in("ax") value,
            in("dx") port,
        );
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let mut result: u8;

    unsafe {
        asm!(
            "inb dx, al",
            out("al") result,
            in("dx") port,
        );
    };
    result
}

pub unsafe fn inw(port: u16) -> u16 {
    let mut result: u16;

    unsafe {
        asm!(
            "inb dx, ax",
            out("ax") result,
            in("dx") port,
        );
    };
    result
}
