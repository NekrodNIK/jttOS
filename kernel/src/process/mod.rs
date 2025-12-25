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
    x86_utils::EFlags,
};
use core::{
    arch::asm,
    cell::{LazyCell, RefCell},
};

pub static mut PROCESSES: nullsync::LazyCell<[Process; 4]> = nullsync::LazyCell::new(|| {
    [
        template_process(0x800_000, 0, 0, 2, 2),
        template_process(0x810_000, 1, 0, 2, 2),
        template_process(0x820_000, 0, 1, 2, 2),
        template_process(0x830_000, 1, 0, 2, 2),
    ]
});

pub fn get_process(pid: u32) -> Option<&'static mut Process> {
    match pid {
        0..3 => Some(unsafe { &mut PROCESSES[pid as usize] }),
        _ => None,
    }
}

pub fn template_process(
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

    Process::new(as_fn(addr as _), TextBufferWritter::new(process_tb))
}

pub struct Process {
    pub pid: u32,
    pub state: u8,
    pub entry: fn(),
    pub tbw: TextBufferWritter,
    pub ctx: InterruptContext,
    pub pd: *mut paging::PageDirectory,
}

static mut CUR_MAX_PID: u32 = 0;

impl Process {
    pub fn new(entry: fn(), tbw: TextBufferWritter) -> Self {
        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);

        Self {
            pid: unsafe {
                let res = CUR_MAX_PID;
                CUR_MAX_PID += 1;
                res
            },
            state: 0,
            entry: entry,
            tbw,
            ctx: InterruptContext {
                esp: entry as _,
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
            },
            pd: unsafe { paging::PAGE_DIRECTORY },
        }
    }

    pub fn run(&mut self, args: &[&[u8]]) {
        self.init(args);
        self.jump()
    }

    pub fn kill(&mut self) -> ! {
        paging::disable_paging();
        // paging::delete_process_pages(self.pd);
        self.init(&[b"smth"]);
        self.jump();
    }

    pub fn init(&mut self, args: &[&[u8]]) {
        paging::disable_paging();
        paging::init_stack_pages(self.pd);
        let (argc, argv) = paging::init_args_pages(self.pd, args);

        interrupts::register_handler(0xe, process::pagefault_handler);

        self.ctx.eax = argc;
        self.ctx.ecx = argv as _;
    }

    pub fn jump(&self) -> ! {
        paging::enable_paging(self.pd);
        let stack_ctx = self.ctx.clone();
        unsafe {
            asm!("mov ebx, {}", "jmp {}", in(reg) &stack_ctx, in(reg) interrupts::pop_ctx, options(noreturn, nostack));
        }
    }
}
