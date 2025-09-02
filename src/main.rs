#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::Vga;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = Vga::new();
    vga.clear();

    let mut i = 0;

    let mut print = |c| {
        vga[i] = c as u8;
        vga[i + 1] = 0xf;
        i += 2;
    };

    print('H');
    print('e');
    print('l');
    print('l');
    print('o');
    print(' ');
    print('W');
    print('o');
    print('r');
    print('l');
    print('d');
    print('!');

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
