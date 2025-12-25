#![no_std]
#![no_main]

extern crate alloc;

mod critical_section;
mod device_manager;
mod drivers;
mod entry;
mod gdt;
mod global_alloc;
mod interrupts;
mod paging;
mod panic;
mod process;
mod syscalls;
mod tss;
mod x86_utils;

use core::{
    cell::{LazyCell, OnceCell, RefCell},
    mem,
};
use device_manager::DEVICES;
use utils::nullsync;
use utils::textbuffer::TextBufferWritter;
use utils::{as_fn, textbuffer::TextBuffer};
use utils::{io::Write, textbuffer::TextBufferRegion};

use crate::{
    gdt::GDT,
    interrupts::Idt,
    process::{CUR_PROCCESS, PROCESSES, Process},
    tss::TSS,
    x86_utils::{EFlags, cli, sti, tsc_sleep},
};

macro_rules! print {
    ($($arg:tt)*) => {{
        use utils::io::Write;
        write!($crate::TBW.borrow_mut(), $($arg)*).unwrap()
    }};
}

macro_rules! println {
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::print!("[");
        $crate::TBW.borrow_mut().set_next_fg(0x0000ff00);
        $crate::print!("INFO");
        $crate::TBW.borrow_mut().set_next_fg(0x00ffffff);
        $crate::println!("] {}", format_args!($($arg)*));
    }};
}

macro_rules! warning {
    ($($arg:tt)*) => {{
        let mut tbw = $crate::TBW.borrow_mut();
        $crate::print!("[");
        $crate::TBW.borrow_mut().set_next_fg(0x00ffff00);
        $crate::print!("WARNING");
        $crate::TBW.borrow_mut().set_next_fg(0x00ffffff);
        $crate::println!("] {}", format_args!($($arg)*));
    }};
}
pub(crate) use info;
pub(crate) use print;
pub(crate) use println;
pub(crate) use warning;

unsafe extern "C" {
    static framebuffer_addr: *mut u32;
    static framebuffer_width: u16;
    static framebuffer_height: u16;
}

static TBW: nullsync::Marker<LazyCell<RefCell<TextBufferWritter>>> =
    nullsync::Marker::new(LazyCell::new(|| {
        RefCell::new(TextBufferWritter::new(TextBuffer::new(
            utils::framebuffer::Framebuffer {
                addr: unsafe { framebuffer_addr },
                width: unsafe { framebuffer_width as usize },
                height: unsafe { framebuffer_height as usize },
            },
        )))
    }));

pub fn kmain() {
    TBW.borrow_mut().clear();

    let mut idt = Idt::new();
    idt.load();
    info!("IDT loaded");

    GDT.tss.update(|mut tss_desc| {
        tss_desc.set_base(&raw const TSS as u32);
        tss_desc.set_limit((mem::size_of_val(&TSS) - 1) as u32);
        tss_desc
    });

    TSS.load();
    info!("TSS loaded");

    DEVICES.init_devices();

    interrupts::register_handler(0x80, syscalls::generic_handler);
    idt.mark_syscall(0x80);

    sti();
    info!("Interrupts enabled");

    println!(include_str!("logo.txt"));
    println!("Press any key...");

    while DEVICES.ps2keyboard.read() == 0 {
        tsc_sleep(1000);
    }

    TBW.borrow_mut().clear();

    unsafe {
        PROCESSES[0].init(&[b"binary\0", b"--flag=true\0", b"src.c\0", b"a.out\0", b"\0"]);
        PROCESSES[1].init(&[]);
        PROCESSES[2].init(&[]);
        PROCESSES[3].init(&[]);

        interrupts::register_handler(0x20, |ctx| {
            let mut next = (CUR_PROCCESS + 1) % 4;

            while !PROCESSES[next].alive {
                next = (next + 1) % 4;
            }

            PROCESSES[CUR_PROCCESS].ctx = ctx.clone();
            CUR_PROCCESS = next;
            PROCESSES[CUR_PROCCESS].jump();
        });
        cli();
        DEVICES.pic.enable_device(0);
        PROCESSES[0].jump();
    };
}
