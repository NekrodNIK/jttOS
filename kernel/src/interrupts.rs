use core::arch::{asm, naked_asm};
use core::array;
use core::mem;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;

use alloc::boxed::Box;

use crate::x86_utils::{lidt, EFlags};
use crate::TBW;
use utils::io::Write;

static HANDLERS: [AtomicPtr<fn(&InterruptContext)>; 256] =
    [const { AtomicPtr::new(unhandled_panic as _) }; 256];

static USERSPACE_NPE_HANDLER: AtomicPtr<fn(&InterruptContext)> =
    AtomicPtr::new(unhandled_panic as _);
static USERSPACE_SOE_HANDLER: AtomicPtr<fn(&InterruptContext)> =
    AtomicPtr::new(unhandled_panic as _);
static USERSPACE_UB_HANDLER: AtomicPtr<fn(&InterruptContext)> =
    AtomicPtr::new(unhandled_panic as _);

pub struct Idt {
    table: [InterruptDescriptor; 256],
    desc: IdtDescriptor,
}

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct InterruptDescriptor {
    offset0: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset1: u16,
}

#[repr(C, align(4))]
#[derive(Debug, Clone)]
pub struct InterruptContext {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub _fill: u32,
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

    pub esp: u32,
    pub ss: u16,
}

impl Idt {
    pub fn new() -> Self {
        let init_fn = |vector| {
            let mut trampoline = Box::new([0u8; 8]);

            // push $index
            trampoline[0] = 0x6a;
            trampoline[1] = vector as u8;

            let ctx_fn = if !Self::has_errcode(vector as u8) {
                collect_ctx_push
            } else {
                collect_ctx
            };
            let offset = ctx_fn as isize - trampoline.as_ptr() as isize - 7;

            // jmp $offset
            trampoline[2] = 0xe9;
            trampoline[3..7].copy_from_slice(&offset.to_le_bytes());
            InterruptDescriptor::new(Box::into_raw(trampoline) as _)
        };

        Self {
            table: array::from_fn(init_fn),
            desc: IdtDescriptor {
                size: (mem::size_of::<[InterruptDescriptor; 256]>() - 1) as u16,
                offset: 0,
            },
        }
    }

    pub fn load(&mut self) {
        self.desc.offset = self.table.as_ptr() as u32;
        unsafe {
            lidt(&self.desc as *const IdtDescriptor as _);
        }
    }

    const fn has_errcode(vector: u8) -> bool {
        matches!(
            vector,
            0x8 | 0xa | 0xb | 0xc | 0xd | 0xe | 0x11 | 0x15 | 0x1d | 0x1E
        )
    }

    pub fn switch_to_interrupt(&mut self, code: usize) {
        self.table[code].flags = 0x8e;
    }

    pub fn mark_syscall(&mut self, code: usize) {
        self.table[code].flags = 0xee;
    }
}

impl InterruptDescriptor {
    pub const fn new(offset: usize) -> Self {
        Self {
            selector: 8, // code segment
            offset0: (offset & 0xffff) as _,
            offset1: (offset >> 16 & 0xffff) as _,
            flags: 0x8e,
            zero: 0,
        }
    }
}

#[unsafe(naked)]
extern "C" fn collect_ctx_push() {
    naked_asm!(
        "push [esp]",
        "jmp {collect}",
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
        "and esp, ~15",
        "push eax",
        // call handler
        "push ebx",
        "cld",
        "call {handler}",
        "jmp {ret_handler}",

        sel = const 16,
        handler = sym global_handler,
        ret_handler = sym pop_ctx
    )
}

#[unsafe(naked)]
pub extern "C" fn pop_ctx() {
    naked_asm!(
        "mov esp, ebx",
        "popad",
        "pop gs",
        "pop fs",
        "pop es",
        "pop ds",
        "add esp, 8",
        "iretd",
    )
}

extern "C" fn global_handler(ctx: *const InterruptContext) {
    if ctx.is_null() {
        panic!("Invalid context passed to global interrupt handler")
    }

    let ctx = unsafe { &*ctx };

    unsafe {
        (mem::transmute::<_, fn(&InterruptContext)>(
            HANDLERS[ctx.vector as usize].load(Ordering::Relaxed),
        ))(ctx);
    }
}

fn unhandled_panic(ctx: &InterruptContext) {
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

pub fn register_handler(index: u8, handler: fn(&mut InterruptContext)) {
    HANDLERS[index as usize].store(handler as _, Ordering::Relaxed);
}

pub fn page_fault_handler(ctx: &mut InterruptContext) {
    let cr2: u32;
    unsafe { asm!("mov {}, cr2", out(reg) cr2) }
    let us_bit = ctx.errcode & (1 << 2) != 0;

    if !us_bit {
        unhandled_panic(ctx);
    }

    enum UserErr {
        NPE,
        SOE,
        UB,
    }

    let user_err = match cr2 {
        0..0x7000 => UserErr::NPE,
        0x80000..0x400000 => UserErr::SOE,
        _ => UserErr::UB,
    };

    let handler = match user_err {
        UserErr::NPE => &USERSPACE_NPE_HANDLER,
        UserErr::SOE => &USERSPACE_SOE_HANDLER,
        UserErr::UB => &USERSPACE_UB_HANDLER,
    }
    .load(Ordering::Relaxed);

    unsafe {
        (mem::transmute::<_, fn(&mut InterruptContext)>(handler))(ctx);
    }
}

pub fn register_userspace_npe_handler(handler: fn(&mut InterruptContext)) {
    USERSPACE_NPE_HANDLER.store(handler as _, Ordering::Relaxed);
}

pub fn register_userspace_soe_handler(handler: fn(&mut InterruptContext)) {
    USERSPACE_SOE_HANDLER.store(handler as _, Ordering::Relaxed);
}
pub fn register_userspace_ub_handler(handler: fn(&mut InterruptContext)) {
    USERSPACE_UB_HANDLER.store(handler as _, Ordering::Relaxed);
}
