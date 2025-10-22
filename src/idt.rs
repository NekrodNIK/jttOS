use core::arch::asm;
use core::mem;
use core::mem::MaybeUninit;

use alloc::boxed::Box;

use crate::io::Write;
use crate::utils::lidt;
use crate::{console, println};

#[repr(transparent)]
pub struct Idt(pub [InterruptDescriptor; 256]);

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct InterruptDescriptor {
    offset0: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset1: u16,
}

impl Idt {
    pub fn new() -> Self {
        let mut table = [MaybeUninit::<InterruptDescriptor>::uninit(); 256];

        for index in 0..256 {
            let mut trampoline = Box::new([0u8; 8]);

            // push $index
            trampoline[0] = 0x6a;
            trampoline[1] = index as u8;

            // jmp collect_ctx
            trampoline[2] = 0xe9;
            let offset = collect_ctx as isize - trampoline.as_ptr() as isize - 7;
            trampoline[3..7].copy_from_slice(&offset.to_le_bytes());

            table[index].write(InterruptDescriptor::new(Box::into_raw(trampoline) as _));
        }

        Self(unsafe { mem::transmute(table) })
    }

    pub fn load(&self) {
        #[repr(C, packed)]
        struct Desc {
            size: u16,
            offset: u32,
        }

        let desc = Box::new(Desc {
            size: 256 * mem::size_of::<InterruptDescriptor>() as u16 - 1,
            offset: self.0.as_ptr() as u32,
        });

        unsafe {
            lidt(Box::into_raw(desc) as _);
        }
    }
}

impl InterruptDescriptor {
    pub const fn new(offset: usize) -> Self {
        Self {
            selector: 8, // code segment
            offset0: (offset & 0xffff) as _,
            offset1: (offset >> 16 & 0xffff) as _,
            flags: 0b1_00_0_1110,
            zero: 0,
        }
    }
}

pub extern "C" fn collect_ctx() -> ! {
    panic!("YEP!");
}
