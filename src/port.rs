// https://wiki.osdev.org/Inline_Assembly/Examples#I.2FO_access
#![allow(dead_code)]
use core::{arch::asm, marker::PhantomData};

pub struct Port<T> {
    address: u16,
    _mark: PhantomData<T>,
}

pub trait PortIn {
    unsafe fn port_in(port: u16) -> Self;
}

pub trait PortOut {
    unsafe fn port_out(port: u16, value: Self);
}

impl<T> Port<T> {
    pub const fn new(address: u16) -> Self {
        Self {
            address,
            _mark: PhantomData,
        }
    }

    pub fn address(&self) -> u16 {
        self.address
    }
}

impl<T: PortIn> Port<T> {
    pub fn read(&self) -> T {
        unsafe { T::port_in(self.address) }
    }
}

impl<T: PortOut> Port<T> {
    pub fn write(&self, data: T) {
        unsafe { T::port_out(self.address, data) }
    }
}

impl PortIn for u8 {
    unsafe fn port_in(port: u16) -> Self {
        unsafe { inb(port) }
    }
}

impl PortIn for u16 {
    unsafe fn port_in(port: u16) -> Self {
        unsafe { inw(port) }
    }
}

impl PortIn for u32 {
    unsafe fn port_in(port: u16) -> Self {
        unsafe { inl(port) }
    }
}

impl PortOut for u8 {
    unsafe fn port_out(port: u16, value: Self) {
        unsafe { outb(port, value) }
    }
}

impl PortOut for u16 {
    unsafe fn port_out(port: u16, value: Self) {
        unsafe { outw(port, value) }
    }
}

impl PortOut for u32 {
    unsafe fn port_out(port: u16, value: Self) {
        unsafe { outl(port, value) }
    }
}

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("al") value,
            in("dx") port,
        )
    }
}

pub unsafe fn outw(port: u16, value: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("ax") value,
            in("dx") port,
        );
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let mut result: u8;

    unsafe {
        asm!(
            "in al, dx",
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
            "in ax, dx",
            out("ax") result,
            in("dx") port,
        );
    };
    result
}

pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;

    unsafe {
        asm!(
            "in eax, dx",
            out("eax") result,
            in("dx") port,
        );
    }
    result
}
