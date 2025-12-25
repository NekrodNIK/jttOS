mod errors;
pub use errors::pagefault_handler;
use utils::{
    as_fn, nullsync,
    textbuffer::{TextBufferRegion, TextBufferWritter},
};

use crate::{
    TBW,
    gdt::{USER_CS, USER_DS},
    interrupts::{self, InterruptContext},
    paging::{self, POOL4K},
    println, process,
    x86_utils::{EFlags, cli, tsc_sleep},
};
use core::{
    arch::asm,
    cell::{LazyCell, RefCell},
    ptr::{null, null_mut},
};

pub const VIRT_START: *mut u8 = 0x800_000 as _;

pub static mut PROCESSES: nullsync::LazyCell<[Process; 4]> = nullsync::LazyCell::new(|| {
    [
        template_process(0, 0x20000, 0, 0, 2, 2),
        template_process(1, 0x30000, 1, 0, 2, 2),
        template_process(2, 0x40000, 0, 1, 2, 2),
        template_process(3, 0x50000, 1, 1, 2, 2),
    ]
});
pub static mut CUR_PROCCESS: usize = 0;

pub fn get_cur_process() -> &'static mut Process {
    unsafe { &mut PROCESSES[CUR_PROCCESS] }
}

pub fn template_process(
    pid: usize,
    addr: usize,
    x: usize,
    y: usize,
    width_factor: usize,
    height_factor: usize,
) -> Process {
    let kernel_tb = &TBW.borrow_mut().buffer;

    let split_x = kernel_tb.fb.width / width_factor;
    let split_y = kernel_tb.fb.height / height_factor;

    let process_tb = kernel_tb.sub(TextBufferRegion {
        x: x * split_x,
        y: y * split_y,
        width: split_x,
        height: split_y,
    });

    Process::new(pid, addr as _, TextBufferWritter::new(process_tb))
}

pub struct Process {
    pub pid: usize,
    pub alive: bool,
    pub tbw: TextBufferWritter,
    pub ctx: InterruptContext,
    pub pd: *mut paging::PageDirectory,
}

impl Process {
    pub fn new(pid: usize, phys_start: *mut u8, tbw: TextBufferWritter) -> Self {
        let pd = paging::init_kernel_paging();
        paging::init_code_pages(pd, phys_start);

        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);

        Self {
            pid,
            alive: true,
            tbw,
            ctx: InterruptContext {
                esp: VIRT_START as _,
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
                eip: VIRT_START as _,
                cs: USER_CS,
                eflags: flags,
                cr2: 0,
            },
            pd,
        }
    }

    pub fn run(&mut self, args: &[&[u8]]) -> ! {
        self.init(args);
        self.jump()
    }

    pub fn kill(&mut self) {
        paging::disable_paging();
        paging::delete_process_pages(self.pd);
        self.alive = false;
    }

    pub fn respawn(&mut self) -> ! {
        self.kill();
        self.run(&[b"smth"]);
    }

    pub fn init(&mut self, args: &[&[u8]]) {
        paging::disable_paging();
        paging::init_stack_pages(self.pd);
        let (argc, argv) = paging::init_args_pages(self.pd, args);

        interrupts::register_handler(0xe, process::pagefault_handler);

        self.ctx.eax = argc;
        self.ctx.ecx = argv as _;
    }

    pub fn jump(&mut self) -> ! {
        self.alive = true;
        paging::enable_paging(self.pd);
        let stack_ctx = self.ctx.clone();
        unsafe {
            asm!("mov ebx, {}", "jmp {}", in(reg) &stack_ctx, in(reg) interrupts::pop_ctx, options(noreturn, nostack));
        }
    }
}
