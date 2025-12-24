use crate::{
    gdt::{USER_CS, USER_DS},
    interrupts::{self, InterruptContext},
    paging,
    x86_utils::EFlags,
};
use core::arch::asm;

const STACK: *mut u8 = 0x800_000 as _;

pub struct Process {
    pub pid: u32,
    pub state: u8,
    pub entry: fn(),
}

static mut CUR_MAX_PID: u32 = 0;

impl Process {
    pub fn new(entry: fn()) -> Self {
        Self {
            pid: unsafe {
                let res = CUR_MAX_PID;
                CUR_MAX_PID += 1;
                res
            },
            state: 0,
            entry: entry,
        }
    }

    pub fn run(&self) {
        // paging::disable_paging();
        // paging::init_user_paging();
        // paging::enable_user_pages(0x400_000 as _);
        // paging::enable_paging();

        jump_to_userspace(self.entry, STACK);
    }
}

pub fn jump_to_userspace(entry: fn(), stack: *mut u8) {
    let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);

    let ctx = InterruptContext {
        esp: stack as _,
        ss: USER_DS,
        edi: 0,
        esi: 0,
        ebp: 0,
        _fill: 0,
        ebx: 0,
        edx: 0,
        ecx: 0,
        eax: 0,
        gs: USER_DS,
        fs: USER_DS,
        es: USER_DS,
        ds: USER_DS,
        vector: 0,
        errcode: 0,
        eip: entry as _,
        cs: USER_CS,
        eflags: flags,
        cr2: 0,
    };

    unsafe {
        asm!("mov ebx, {}", "jmp {}", in(reg) &ctx, in(reg) interrupts::pop_ctx);
    }
}
