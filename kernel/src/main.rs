#![no_std]
#![no_main]
#![feature(const_trait_impl)]

extern crate alloc;

mod critical_section;
mod device_manager;
mod drivers;
mod entry;
mod gdt;
mod global_alloc;
mod interrupts;
mod lab7;
mod paging;
mod panic;
mod syscalls;
mod tss;
mod x86_utils;

use alloc::boxed::Box;
use core::{
    mem,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};
use device_manager::DEVICES;
use utils::{io::Write, textbuffer::TextBufferWritter};

use crate::{
    gdt::GDT,
    interrupts::{Idt, InterruptContext},
    tss::TSS,
    x86_utils::{EFlags, tsc_sleep},
};
use core::arch::asm;

#[macro_export]
macro_rules! info {
    ($tbw:expr, $($arg:tt)*) => {{
        write!($tbw, "[").unwrap();
        $tbw.set_next_fg(0x0000ff00);
        write!($tbw, "INFO").unwrap();
        $tbw.set_next_fg(0x00ffffff);
        write!($tbw, "] {}\n", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! warning {
    ($tbw:expr, $($arg:tt)*) => {{
        write!($tbw, "[").unwrap();
        $tbw.set_next_fg(0x00ffff00);
        write!($tbw, "WARNING").unwrap();
        $tbw.set_next_fg(0x00ffffff);
        write!($tbw, "] {}\n", format_args!($($arg)*)).unwrap();
    }};
}

unsafe extern "C" {
    static framebuffer_addr: *mut u32;
    static framebuffer_width: u16;
    static framebuffer_height: u16;
}

fn new_tbw() -> utils::textbuffer::TextBufferWritter {
    let tbw = unsafe {
        utils::textbuffer::TextBufferWritter::new(utils::textbuffer::TextBuffer::new(
            utils::framebuffer::Framebuffer {
                addr: framebuffer_addr,
                width: framebuffer_width as usize,
                height: framebuffer_height as usize,
            },
        ))
    };
    tbw
}

// lab7 code -> DELETE AFTER LAB 7
static mut TBW: *mut TextBufferWritter = 0 as _; // :(
//

pub fn kmain() {
    let mut tbw = new_tbw();
    tbw.clear();

    // lab7 code -> DELETE AFTER LAB 7
    static TBW: AtomicPtr<TextBufferWritter> = AtomicPtr::new(0 as _);
    TBW.store(&raw mut tbw, Ordering::Release);
    if cfg!(labs) {
        lab7::run()
    } else {
        paging::init_kernel_paging(
            paging::PageDirectoryEntry::new_4mb(0 as _, true, true, true),
            true,
        );
        paging::enable_paging();
    }
    //

    info!(tbw, "Paging enabled");

    let mut idt = Idt::new();
    info!(tbw, "IDT loaded");
    idt.mark_syscall(0x30);
    idt.load();
    GDT.tss.update(|mut tss_desc| {
        tss_desc.set_base(&raw const TSS as u32);
        tss_desc.set_limit((mem::size_of_val(&TSS) - 1) as u32);
        tss_desc
    });

    TSS.load();
    info!(tbw, "TSS loaded");

    DEVICES.init_devices(&mut tbw);

    interrupts::register_handler(0x80, syscalls::generic_handler);
    idt.mark_syscall(0x80);
    unsafe { x86_utils::sti() };
    info!(tbw, "Interrupts enabled");

    writeln!(tbw, include_str!("logo.txt")).unwrap();
    writeln!(tbw, "Press any key...").unwrap();

    while DEVICES.ps2keyboard.read() == 0 {
        tsc_sleep(10000)
    }

    // lab7 artifact -> DELETE AFTER LAB 7
    if cfg!(ex1) || cfg!(ex2) || cfg!(ex3) || cfg!(ex4) || cfg!(ex5) || cfg!(ex6) {
        unsafe {
            static mut X: usize = 0;

            interrupts::register_handler(0x30, |ctx| {
                write!(*TBW.load(Ordering::Relaxed), "{} ", ctx.eax).unwrap()
            });
            interrupts::register_handler(0x20, |_| X = 0);
            DEVICES.pic.enable_device(0);

            jump_to_userspace(
                || {
                    loop {
                        asm!("int 0x30", in("eax") X);
                        X += 1;
                    }
                },
                null_mut(),
            );
        }
    } else if cfg!(ex7) {
        jump_to_userspace(
            || unsafe {
                write!(*TBW.load(Ordering::Relaxed), "USERSPACE").unwrap();
                loop {}
            },
            null_mut(),
        );
    } else if cfg!(ex8) {
        unsafe {
            static mut X: usize = 0;

            interrupts::register_handler(0x30, |ctx| {
                write!(*TBW.load(Ordering::Relaxed), "{} ", ctx.eax).unwrap()
            });
            interrupts::register_handler(0x20, |_| X = 0);
            DEVICES.pic.enable_device(0);

            jump_to_userspace(
                || {
                    loop {
                        asm!("int 0x30", in("eax") X);
                        X += 1;
                    }
                },
                0x800_000 as _,
            );
        }
    } else if cfg!(ex9) {
        static mut X: u32 = 0;

        fn user_exit(code: u32) {
            unsafe {
                asm!("int 0x30", in("eax") code);
            }
        }

        fn syscall_exit(ctx: &mut InterruptContext) {
            paging::disable_paging();
            paging::init_user_paging();
            paging::enable_paging();

            unsafe {
                write!(*TBW.load(Ordering::Relaxed), "{} ", u32::from(X)).unwrap();
                X += 1
            }

            jump_to_userspace(
                || unsafe {
                    user_exit(X);
                    loop {}
                },
                0x800_000 as _,
            );
        }

        interrupts::register_handler(0x30, syscall_exit);

        jump_to_userspace(
            || unsafe {
                user_exit(X);
                loop {}
            },
            0x800_000 as _,
        );
    }
    //

    if !cfg!(labs) {
        jump_to_userspace(userspace::entry, null_mut());
    }
}

pub fn jump_to_userspace(entry: fn(), mut stack: *mut u8) {
    unsafe {
        if stack.is_null() {
            stack = Box::into_raw(Box::new([0u8; 4 * 1024])) as _;
        }
        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);

        let cs = 24 | 0b11;
        let ds = 32 | 0b11;

        let ctx = InterruptContext {
            esp: stack.add(4 * 1024) as _,
            ss: ds,
            edi: 0,
            esi: 0,
            ebp: 0,
            _fill: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
            eax: 0,
            gs: ds,
            fs: ds,
            es: ds,
            ds: ds,
            vector: 0,
            errcode: 0,
            eip: entry as _,
            cs: cs,
            eflags: flags,
        };

        asm!("mov ebx, {}", "jmp {}", in(reg) &ctx,  in(reg) interrupts::pop_ctx);
    }
}
