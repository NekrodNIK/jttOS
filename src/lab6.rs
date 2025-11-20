#![allow(unexpected_cfgs)]
use crate::device_manager::DEVICES;
use crate::io::Write;
use crate::{console, interrupts, userspace, utils};
use core::arch::asm;
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn run() {
    if cfg!(ex1) {
        ex1()
    } else if cfg!(ex2) {
        ex2()
    } else if cfg!(ex3) {
        ex3()
    } else if cfg!(ex4) {
        ex4()
    } else if cfg!(ex5) {
        ex5()
    } else if cfg!(ex6) {
        ex6()
    } else if cfg!(ex7) {
        ex7()
    } else if cfg!(ex8) {
        ex8()
    }

    loop {}
}

fn ex1() {
    userspace::run(|| loop {});
}

fn ex2() {
    userspace::run(|| {
        console::println!("Userspace");
        loop {}
    });
}

fn ex3() {
    static X: AtomicUsize = AtomicUsize::new(0);
    userspace::run(|| {
        loop {
            console::println!("{}", X.fetch_add(1, Ordering::Relaxed));
        }
    });
}

fn ex4() {
    userspace::run(|| {
        let mut esp: u32;
        unsafe { asm!("mov {}, esp", out(reg) esp) };
        console::println!("ESP: {:#x}", esp);
        loop {}
    });
}

fn ex5() {
    userspace::run(|| {
        unsafe {
            // asm!("cli");
            // asm!("sti");
            // asm!("lgdt [eax]");
            // asm!("ltr [eax]")
            asm!("mov cr0, eax");
        };
        loop {}
    })
}

fn ex6() {
    static X: AtomicUsize = AtomicUsize::new(0);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("ESP: {:#x}", utils::esp());
        loop {}
    });

    userspace::run(|| {
        loop {
            console::print!("{} ", X.fetch_add(1, Ordering::Relaxed));
        }
    });
}

fn ex7() {
    static mut X: usize = 0;
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| unsafe { X = 0 });

    userspace::run(|| unsafe {
        loop {
            let y = X;
            console::print!("{} ", y);
            X += 1;
        }
    });
}

fn ex8() {
    static mut X: usize = 1;
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| unsafe { X = 0 });

    userspace::run(|| unsafe {
        loop {
            let y = X;
            console::print!("{} ", y);
            X += 1;
        }
    });
}

fn ex9() {
    userspace::run(|| unsafe {});
}
