use core::arch::{asm, naked_asm};
use core::cell::OnceCell;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::DerefMut;

use alloc::boxed::Box;

use crate::io::Write;
use crate::println;
use crate::sync::IrqSafe;
use crate::utils::{EFlags, lidt};

#[repr(transparent)]
pub struct Idt([InterruptDescriptor; 256]);

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct InterruptDescriptor {
    offset0: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset1: u16,
}

#[repr(C, align(4))]
#[derive(Debug, Clone)]
struct InterruptContext {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,

    pub gs: u16,
    pub fs: u16,
    pub es: u16,
    pub ds: u16,

    pub vector: u8,

    pub errcode: u32,
    pub eip: u32,
    pub cs: u16,
    pub eflags: EFlags,
}

impl Idt {
    pub fn new() -> Self {
        let mut table = [MaybeUninit::<InterruptDescriptor>::uninit(); 256];

        for vector in 0..=255 {
            let mut trampoline = Box::new([0u8; 8]);

            // push $index
            trampoline[0] = 0x6a;
            trampoline[1] = vector;

            let ctx_fn = if !Self::has_errcode(vector) {
                collect_ctx_push
            } else {
                collect_ctx
            };
            let offset = ctx_fn as isize - trampoline.as_ptr() as isize - 7;

            // jmp $offset
            trampoline[2] = 0xe9;
            trampoline[3..7].copy_from_slice(&offset.to_le_bytes());
            table[vector as usize].write(InterruptDescriptor::new(Box::into_raw(trampoline) as _));
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

    #[inline(always)]
    pub const fn has_errcode(vector: u8) -> bool {
        match vector {
            0x8 | 0xa | 0xb | 0xc | 0xd | 0xe | 0x11 | 0x15 => true,
            _ => false,
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

#[unsafe(naked)]
extern "C" fn collect_ctx_push() {
    naked_asm!(
        "push [esp]", "jmp {collect}",
        collect = sym collect_ctx
    );
}

#[unsafe(naked)]
extern "C" fn collect_ctx() {
    naked_asm!(
        // save registers
        "push ds",
        "push es",
        "push fs",
        "push gs",
        "pushad",
        // set selectors
        "mov ax, {sel}",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        // align
        "mov ebx, esp",
        "and esp, ~7",
        "push eax",
        // call handler
        "push ebx",
        "cld",
        "call {handler}",

        sel = const 16,
        handler = sym interrupt_handler,
    )
}

extern "C" fn interrupt_handler(ctx: *const InterruptContext) {
    let ctx = unsafe { (*ctx).clone() };

    panic!(
        concat!(
            "unhandled interrupt #{} at {:#x}:{:#x}\n",
            "\nREGISTERS\n",
            "    eax: {:#x}\n",
            "    ecx: {:#x}\n",
            "    edx: {:#x}\n",
            "    ebx: {:#x}\n",
            "    esp: {:#x}\n",
            "    ebp: {:#x}\n",
            "    esi: {:#x}\n",
            "    edi: {:#x}\n",
            "    ds:  {:#x}\n",
            "    es:  {:#x}\n",
            "    fs:  {:#x}\n",
            "    gs:  {:#x}\n\n",
            "ERROR_CODE\n",
            "    value: {:#x}\n\n",
            "EFLAGS\n",
            "    value: {:?}\n",
            "    raw:   {:#x}\n",
        ),
        ctx.vector,
        ctx.cs,
        ctx.eip,
        ctx.eax,
        ctx.ecx,
        ctx.edx,
        ctx.ebx,
        ctx.esp,
        ctx.ebp,
        ctx.esi,
        ctx.edi,
        ctx.ds,
        ctx.es,
        ctx.fs,
        ctx.gs,
        ctx.errcode,
        ctx.eflags,
        ctx.eflags
    );
}
