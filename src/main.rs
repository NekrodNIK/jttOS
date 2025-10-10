#![no_std]
#![no_main]
extern crate alloc;

mod allocator;
mod console;
mod io;
mod irq;
mod utils;

use alloc::alloc::alloc;
use alloc::boxed::Box;
use alloc::slice;
use alloc::string::ToString;

use crate::io::Write;
use crate::utils::{cli, tsc_sleep};
use core::alloc::Layout;
use core::panic::PanicInfo;

const LOGO: &str = include_str!("logo.txt");

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    console::clear!();

    console::println!("{}", LOGO);
    console::info!("{}", "Loading system...");

    let sizes: [usize; 5] = [1, 3, 5, 7, 9];
    let aligns: [usize; 5] = [2, 4, 8, 16, 32];

    for item in sizes.iter().zip(aligns.iter()).cycle() {
        let (size, align) = item;
        let p = unsafe { alloc(Layout::from_size_align(*size, *align).unwrap()) };

        console::info!("align: {}, size: {}, address: {:x?}", align, size, p);

        tsc_sleep(1000000000);
    }

    // DEMO
    // let mut index = 0;
    // loop {
    //     console::info!("I'm scrolling! index: {}", index);
    //     index += 1;
    //     tsc_sleep(20000000);
    // }

    // debug_assert_eq!(
    //     "Answer to the Ultimate Question of Life, the Universe, and Everything",
    //     "42"
    // );

    // panic!("Some panic message");

    // let mut string = "one".to_string();
    // console::info!("Original: \"{}\", size: {} bytes", string, string.len());
    // string += " + another";
    // console::info!("After concat: \"{}\", size: {} bytes", string, string.len());

    // let layout = Layout::from_size_align(1, 8).unwrap();

    // let ptr1;
    // let ptr2;
    // unsafe {
    //     ptr1 = alloc(layout);
    //     ptr2 = alloc(layout);

    //     *ptr1 = 0x1;
    //     *ptr2 = 0x2;

    //     console::info!(
    //         "ptr1, memory location: {:x}, value {:?}",
    //         ptr1 as usize,
    //         slice::from_raw_parts(ptr1, 16)
    //     );

    //     console::info!(
    //         "ptr2, memory location: {:x}, value {:x?}",
    //         ptr2 as usize,
    //         slice::from_raw_parts(ptr2, 16)
    //     );
    // };

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }
    console::CONSOLE.try_unlock();
    console::clear!();
    console::println!("[{}]", red!("KERNEL PANIC"));
    console::print!("{}", info.message());
    loop {}
}
