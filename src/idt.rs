#![allow(dead_code)]
use core::arch::asm;
use paste::paste;
use seq_macro::seq;

pub struct Idt(pub [InterruptDescriptor; 256]);

impl Idt {
    pub fn new() -> Self {
        let mut obj = Self([InterruptDescriptor::default(); 256]);

        seq!(N in 0..=255 {
            obj.0[N].set_entry(paste!([<redirector N>]) as u32);
        });

        obj
    }

    pub unsafe fn load(&self) {
        unsafe {
            asm!("mov eax, {ptr}",
                 "lidt [eax]",
                 ptr = in(reg) self.0.as_ptr());
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(packed)]
pub struct InterruptDescriptor {
    offset_low: u16,
    selector: u16,
    zero: u8,
    attributes: u8,
    offset_high: u16,
}

impl InterruptDescriptor {
    pub fn set_entry(&mut self, entry: u32) {
        let address = entry;
        self.offset_low = (address & 0xffff) as u16;
        self.offset_high = (address >> 16) as u16;
        self.attributes = 0x8e;
    }
}

pub fn interrupt_handler(id: u8) {
    match id {
        _ => panic!("IRQ{}", id),
    }
}

#[repr(C)]
struct InterruptStackFrame {
    instruction_pointer: u32,
    code_segment: u32,
    flags: u32,
}

macro_rules! gen_redirector {
    ($id:literal) => {
        paste! {
            extern "x86-interrupt" fn [<redirector $id>](_: InterruptStackFrame) {
                interrupt_handler($id);
            }
        }
    };
}

seq!(N in 0..=255 {
    gen_redirector!(N);
});
