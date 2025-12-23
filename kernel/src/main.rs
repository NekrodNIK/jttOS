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
    paging::{disable_paging, enable_paging, enable_user_pages},
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
static TBW: AtomicPtr<TextBufferWritter> = AtomicPtr::new(0 as _); // :(
//

pub fn kmain() {
    let mut tbw = new_tbw();
    tbw.clear();

    // lab7 code -> DELETE AFTER LAB 7
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
    } else if cfg!(ex10) {
        static mut X: u32 = 0;
        fn user_exit(code: u32) {
            unsafe {
                asm!("int 0x30", in("eax") code);
            }
        }

        fn user_main() {
            unsafe {
                match X % 4 {
                    0 => user_exit(1),
                    1 => asm!("mov eax, [0x42]"),
                    2 => asm!("2: sub esp, 4092", "call 2b"),
                    3 => *(0x900000 as *mut u32) = 1,
                    _ => (),
                }
            }
            loop {}
        }

        fn reload_process() {
            paging::disable_paging();
            paging::init_user_paging();
            paging::enable_paging();
            jump_to_userspace(user_main, 0x800_000 as _);
        }

        fn syscall_exit(ctx: &mut InterruptContext) {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "{} ", u32::from(X)).unwrap();
                X += 1
            }

            reload_process()
        }

        interrupts::register_handler(0x30, syscall_exit);
        interrupts::register_handler(0xe, interrupts::page_fault_handler);
        interrupts::register_userspace_npe_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "NPE").unwrap();
                X += 1
            }
            reload_process()
        });
        interrupts::register_userspace_soe_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "SOE").unwrap();
                X += 1
            }
            reload_process()
        });
        interrupts::register_userspace_ub_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "UB").unwrap();
                X += 1;
            }
            reload_process()
        });

        jump_to_userspace(user_main, 0x800_000 as _);
    } else if cfg!(ex11) {
        const X: usize = 2000;

        fn user_main() {
            unsafe {
                asm!(
                    "mov edi, {}",
                    "call 2f",
                    "jmp 3f",
                    "2:",
                    "sub esp, 4096",
                    "test edi, edi",
                    "jz 3f",
                    "dec edi",
                    "call 2b",
                    "3:",
                    const X
                )
            }

            loop {}
        }

        fn reload_process() {
            paging::disable_paging();
            paging::init_user_paging();
            paging::enable_paging();
            jump_to_userspace(user_main, 0x800_000 as _);
        }

        interrupts::register_handler(0xe, interrupts::page_fault_handler);
        interrupts::register_userspace_npe_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "NPE").unwrap();
            }
            reload_process()
        });
        interrupts::register_userspace_soe_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "SOE").unwrap();
            }
            reload_process()
        });
        interrupts::register_userspace_ub_handler(|_| {
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "UB").unwrap();
            }
            reload_process()
        });
        interrupts::register_userspace_stack_endpand_handler(|ctx: &mut InterruptContext| {
            disable_paging();
            enable_user_pages(ctx.cr2 as *mut u8);
            unsafe {
                writeln!(*TBW.load(Ordering::Relaxed), "{}", ctx.cr2).unwrap();
            }
            enable_paging();
        });

        jump_to_userspace(user_main, 0x800_000 as _);
    }
    //

    if !cfg!(labs) {
        jump_to_userspace(userspace::entry, null_mut());
    }
}

pub fn jump_to_userspace(entry: fn(), mut stack: *mut u8) {
    unsafe {
        if stack.is_null() {
            stack = Box::into_raw(Box::new([0u8; 4 * 1024])).add(4 * 1024) as _;
        }
        let flags = EFlags::new().union(EFlags::IOPL0).union(EFlags::IF);

        let cs = 24 | 0b11;
        let ds = 32 | 0b11;

        let ctx = InterruptContext {
            esp: stack as _,
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
            cr2: 0,
        };

        asm!("mov ebx, {}", "jmp {}", in(reg) &ctx,  in(reg) interrupts::pop_ctx);
    }
}
