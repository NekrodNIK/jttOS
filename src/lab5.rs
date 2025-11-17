#![allow(unexpected_cfgs)]

use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::boxed::Box;

use crate::device_manager::DEVICES;
use crate::drivers::port::Port;
use crate::drivers::ps2;
use crate::interrupts::Idt;
use crate::io::Write;
use crate::utils::sti;
use crate::{console, interrupts};

static X: AtomicUsize = AtomicUsize::new(0);
static PARSER: ps2::KeyParser = ps2::KeyParser::new();

pub fn run() {
    DEVICES.init_devices();

    let mut idt = Box::new(Idt::new());
    idt.load();
    console::info!("IDT loaded");
    console::info!("Run experiment...");

    if cfg!(ex1) {
        run_ex1()
    } else if cfg!(ex2) {
        run_ex2()
    } else if cfg!(ex3) {
        run_ex3()
    } else if cfg!(ex4) {
        run_ex4()
    } else if cfg!(ex5) {
        run_ex5()
    } else if cfg!(ex6) {
        run_ex6()
    } else if cfg!(ex7) {
        run_ex7()
    } else if cfg!(ex8) {
        run_ex8()
    } else if cfg!(ex9) {
        run_ex9()
    } else if cfg!(ex10) {
        run_ex10()
    } else if cfg!(ex11) {
        run_ex11(&mut idt)
    } else if cfg!(ex12) {
        run_ex12()
    } else if cfg!(ex13) {
        run_ex13()
    } else if cfg!(ex14) {
        run_ex14()
    } else if cfg!(ex15) {
        run_ex15()
    } else if cfg!(ex16) {
        run_ex16()
    } else if cfg!(ex17) {
        run_ex17()
    } else if cfg!(ex18) {
        run_ex18(&mut idt)
    } else if cfg!(ex19) {
        run_ex19(&mut idt)
    } else if cfg!(ex20) {
        run_ex20();
    }
}

fn run_ex1() {
    DEVICES.pic.init(false);
    unsafe { sti() };
}

fn run_ex2() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);
    unsafe { sti() };
}

fn run_ex3() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(1);
    unsafe { sti() };
}

fn run_ex4() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed))
    });
    unsafe { sti() };
}

fn run_ex5() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        X.swap(0, Ordering::Relaxed);
    });

    unsafe { sti() };
    loop {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed));
    }
}

fn run_ex6() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed))
    });
    unsafe { sti() };
    loop {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed));
    }
}

fn run_ex7() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed))
    });
    unsafe { sti() };
}

fn run_ex8() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed));
        DEVICES.pic.send_eoi(0);
    });

    unsafe { sti() };
}

fn run_ex9() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        console::print!("{} ", X.fetch_add(1, Ordering::Relaxed));
        DEVICES.pic.send_eoi(0);
        unsafe { sti() }
        loop {}
    });

    unsafe { sti() };
}

fn run_ex10() {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
        if x < 10 {
            DEVICES.pic.send_eoi(0);
        }

        unsafe { sti() }
        loop {}
    });

    unsafe { sti() };
}

fn run_ex11(idt: &mut Idt) {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);

    idt.switch_to_trap(0x20);
    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
        if x < 10 {
            DEVICES.pic.send_eoi(0);
        }

        loop {}
    });

    unsafe { sti() };
}

fn run_ex12() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);

    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
        if x < 10 {
            unsafe {
                sti();
            }
        }

        loop {}
    });

    unsafe { sti() };
}

fn run_ex13() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
        };
    });

    unsafe { sti() };
}

fn run_ex14() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
        };
    });

    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
    });

    unsafe { sti() };
}

fn run_ex15() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
        };
    });

    interrupts::register_handler(0x20, |_| {
        console::println!("{}", X.load(Ordering::Relaxed));
        loop {}
    });

    unsafe { sti() };
}

fn run_ex16() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
            loop {}
        };
    });

    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
    });

    unsafe { sti() };
}

fn run_ex17() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
            unsafe { sti() }
            loop {}
        };
    });

    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
    });

    unsafe { sti() };
}

fn run_ex18(idt: &mut Idt) {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    idt.switch_to_trap(0x21);
    interrupts::register_handler(0x21, |_| {
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
            DEVICES.pic.send_eoi(1);
        };
    });

    idt.switch_to_trap(0x20);
    interrupts::register_handler(0x20, |_| {
        let x = X.load(Ordering::Relaxed);
        console::println!("{}", x);
        loop {}
    });

    unsafe { sti() };
}

fn run_ex19(idt: &mut Idt) {
    DEVICES.pic.init(false);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    idt.switch_to_trap(0x21);
    interrupts::register_handler(0x21, |_| {
        // One byte
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
        }
        loop {}
    });

    idt.switch_to_trap(0x20);
    interrupts::register_handler(0x20, |_| {
        let x = X.fetch_add(1, Ordering::Relaxed);
        console::println!("{}", x);
        DEVICES.pic.send_eoi(0);
    });

    unsafe { sti() };
}

fn run_ex20() {
    DEVICES.pic.init(true);
    DEVICES.pic.enable_device(0);
    DEVICES.pic.enable_device(1);

    interrupts::register_handler(0x21, |_| {
        // One byte
        if let Ok(event) = PARSER.parse(Port::<u8>::new(0x60).read()) {
            console::println!("{:?}", event);
        }
        loop {}
    });

    interrupts::register_handler(0x20, |_| {
        DEVICES.pic.disable_device(0);

        let wait = |j| {
            for i in 0..1000 {
                console::println!("{}: {}", j, i);
            }
        };

        wait(1);
        unsafe {
            sti();
        }
        wait(2);
    });

    unsafe { sti() };
}
