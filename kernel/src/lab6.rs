#![allow(unexpected_cfgs)]
use crate::device_manager::DEVICES;
use crate::gdt::{GDT, SDFlags0};
use crate::io::Write;
use crate::tss::TSS;
use crate::{console, interrupts, jump_to_userspace, println, utils};
use core::arch::asm;

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
    } else if cfg!(ex9) {
        ex9()
    } else if cfg!(hack) {
        hack()
    } else if cfg!(ex10) {
        ex10()
    }
}

fn ex1() {
    jump_to_userspace(|| loop {});
}

fn ex2() {
    jump_to_userspace(|| {
        println!("Hello Userspace!");
        loop {}
    });
}

fn ex3() {
    static mut X: usize = 0;
    jump_to_userspace(|| {
        loop {
            unsafe { X += 1 };
            println!("{}", { X });
        }
    });
}

fn ex4() {
    jump_to_userspace(|| {
        console::println!("ESP: {:#x}", utils::esp());
        loop {}
    });
}

fn ex5() {
    jump_to_userspace(|| {
        unsafe {
            // asm!("cli");
            // asm!("sti");
            // asm!("lgdt [eax]");
            // asm!("ltr [eax]")
            asm!("mov cr0, eax");
            // Port::<u8>::new(0x80).write(0);
        };
        loop {}
    });
}

fn ex6() {
    interrupts::register_handler(0x20, |_| {
        console::print!("{:#x}", utils::esp());
    });
    DEVICES.pic.enable_device(0);

    static mut X: usize = 0;
    jump_to_userspace(|| {
        loop {
            unsafe { X += 1 };
            console::print!("{} ", { X });
        }
    });
}

fn ex7() {
    static mut X: usize = 0;

    interrupts::register_handler(0x20, |_| unsafe { X = 0 });
    DEVICES.pic.enable_device(0);

    jump_to_userspace(|| {
        loop {
            let y = unsafe { X };
            console::print!("{} ", y);
            unsafe {
                X += 1;
            }
        }
    });
}

fn ex8() {
    static mut X: usize = 1;
    interrupts::register_handler(0x20, |_| unsafe { X = 0 });
    DEVICES.pic.enable_device(0);

    jump_to_userspace(|| {
        loop {
            let y = unsafe { X };
            console::print!("{} ", y);
            unsafe { X += 1 };
        }
    });
}

fn ex9() {
    interrupts::register_handler(0x20, |_| {});
    DEVICES.pic.enable_device(0);

    jump_to_userspace(|| {
        GDT.kernel_code.update(|mut desc| {
            desc.flags0 = SDFlags0::from_bits_retain(desc.flags0)
                .difference(SDFlags0::PRESENT)
                .bits();
            desc
        });
        loop {}
    });
}

fn hack() {
    interrupts::register_handler(0x20, |_| console::clear!());
    DEVICES.pic.enable_device(0);

    jump_to_userspace(|| {
        TSS.iomap_base.set(0);
        DEVICES.pic.disable_device(0);
        console::println!("Broken");
        loop {}
    });
}

fn ex10() {
    static mut X: usize = 0;
    interrupts::register_handler(0x30, |ctx| console::print!("{} ", ctx.eax));
    interrupts::register_handler(0x20, |_| unsafe { X = 0 });
    DEVICES.pic.enable_device(0);

    jump_to_userspace(|| {
        loop {
            unsafe {
                asm!("int 0x30", in("eax") X);
                X += 1;
            }
        }
    });
}
