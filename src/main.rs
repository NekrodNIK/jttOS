#![no_std]
#![no_main]
#![allow(unexpected_cfgs)]
extern crate alloc;

mod allocator;
mod console;
mod devices;
mod interrupts;
mod io;
mod nullsync;
mod pic8259;
mod port;
mod ringbuffer;
mod sync;
mod utils;

use crate::console::CONSOLE;
use crate::devices::{Device, DeviceController};
use core::cell::{Cell, LazyCell, RefCell};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::boxed::Box;
use alloc::rc::Rc;
use core::arch::asm;

use crate::devices::ps2::{PS2Controller, PS2Keyboard};
use crate::interrupts::{Idt, InterruptContext};
use crate::io::Write;
use crate::pic8259::ChainedPics;
use crate::ringbuffer::RingBuffer;
use crate::sync::IntSafe;
use crate::utils::{EFlags, cli, sti, tsc_sleep};

const LOGO: &str = include_str!("logo.txt");

static PICS: IntSafe<ChainedPics> = IntSafe::new(ChainedPics::new(0x20, 0x28));
static PS2: IntSafe<PS2Controller> = IntSafe::new(PS2Controller::new());

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);

    let mut idt = Box::new(Idt::new());
    idt.load();
    console::info!("{}", "IDT loaded");

    PICS.lock().init(false);
    unsafe { sti() };

    #[cfg(ex1)]
    {}
    #[cfg(ex2)]
    PICS.lock().enable_device(0);
    #[cfg(ex3)]
    PICS.lock().enable_device(1);

    // FIXME
    #[cfg(ex4)]
    {
        PICS.lock().init(true);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
        });
    }

    // FIXME
    #[cfg(ex5)]
    {
        static X: AtomicUsize = AtomicUsize::new(0);
        PICS.lock().init(true);
        PICS.lock().enable_device(0);

        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            X.store(0, Ordering::Relaxed);
        });

        loop {
            use crate::utils::tsc_sleep;
            console::print!(
                "{} ",
                X.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(
                    x.wrapping_add(1)
                ))
                .unwrap(),
            );
        }
    }

    // FIXME
    #[cfg(ex6)]
    {
        static X: AtomicUsize = AtomicUsize::new(0);
        PICS.lock().init(true);
        PICS.lock().enable_device(0);

        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            unsafe {
                write!(
                    console::CONSOLE.get(),
                    "{} ",
                    X.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                        Some(x.wrapping_add(1))
                    })
                    .unwrap(),
                );
            };
        });

        loop {
            unsafe {
                write!(
                    console::CONSOLE.get(),
                    "{} ",
                    X.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                        Some(x.wrapping_add(1))
                    })
                    .unwrap(),
                );
            }
        }
    }

    // FIXME
    #[cfg(ex7)]
    {
        PICS.lock().init(false);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
        });
    }

    // FIXME
    #[cfg(ex8)]
    {
        PICS.lock().init(false);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
            PICS.lock().send_eoi(0);
        });
    }

    // FIXME: lockckckckckckjkjdhfd
    #[cfg(ex9)]
    {
        PICS.lock().init(false);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
            PICS.lock().send_eoi(0);

            unsafe {
                sti();
            }

            loop {}
        });
    }

    // FIXME
    #[cfg(ex10)]
    {
        PICS.lock().init(false);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);

            if X.load(Ordering::Relaxed) < 10 {
                PICS.lock().send_eoi(0);
                unsafe {
                    sti();
                }
            }

            loop {}
        });
    }

    // FIXME
    // #[cfg(ex11)]
    {
        idt.set_trap(0x20);
        PICS.lock().init(false);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static mut X: usize = 0;
            writeln!(unsafe { CONSOLE.get() }, "{}", unsafe { X });
            unsafe { X += 1 }

            if unsafe { X < 5 } {
                unsafe {
                    PICS.get().send_eoi(0);
                }
            }

            // loop {}
        });
    }

    // FIXME
    #[cfg(ex12)]
    {
        PICS.lock().init(true);
        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);

            if X.load(Ordering::Relaxed) < 10 {
                unsafe {
                    sti();
                }
            }

            loop {}
        });
    }

    // FIXME
    #[cfg(ex13)]
    {
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx)
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");
    }

    // FIXME
    #[cfg(ex14)]
    {
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx)
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
        });
    }

    // FIXME
    #[cfg(ex15)]
    {
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx)
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);

            loop {}
        });
    }

    // FIXME
    #[cfg(ex16)]
    {
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx);
            loop {}
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
        });
    }

    // FIXME
    #[cfg(ex17)]
    {
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx);
            unsafe { sti() }
            loop {}
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
        });
    }

    // FIXME
    #[cfg(ex18)]
    {
        idt.set_trap(0x21);
        idt.set_trap(0x20);
        PICS.lock().init(false);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx);
            PICS.lock().send_eoi(1);
        });
        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
            loop {}
        });
    }

    // FIXME
    #[cfg(ex19)]
    {
        idt.set_trap(0x21);
        idt.set_trap(0x20);
        PICS.lock().init(false);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx);
            // loop {}
        });

        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            static X: AtomicUsize = AtomicUsize::new(0);
            console::println!("{}", X.load(Ordering::Relaxed));
            X.fetch_add(1, Ordering::Relaxed);
            PICS.lock().send_eoi(0);
        });
        PICS.lock().enable_device(0);
        loop {}
    }

    // FIXME
    #[cfg(ex20)]
    {
        idt.set_trap(0x21);
        idt.set_trap(0x20);
        PICS.lock().init(true);
        PS2.lock().init();
        interrupts::register_handler(0x21, |ctx: &InterruptContext| {
            PS2.lock().keyboard.int_handler(ctx);
            loop {}
        });

        PICS.lock().enable_device(1);
        console::info!("{}", "PS/2 keyboard initialized");
        console::info!("{}", "Interrupts enabled");

        PICS.lock().enable_device(0);
        interrupts::register_handler(0x20, |ctx: &InterruptContext| {
            PICS.lock().disable_device(0);
            for i in 1..1000 {
                console::println!("{}", i);
            }
            unsafe { asm!("sti") }
            for i in 1..1000 {
                console::println!("{}", i);
            }
        });
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }
    console::CONSOLE.try_unlock();
    console::clear!();
    console::println!("[{}]", console::red!("KERNEL PANIC"));
    console::print!("{}", info.message());
    loop {}
}
