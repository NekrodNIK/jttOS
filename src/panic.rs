use core::panic::PanicInfo;

use crate::{console, io::Write, utils::cli};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // unsafe { cli() }
    console::clear!();
    console::println!("[{}]", console::red!("KERNEL PANIC"));
    console::print!("{}", info.message());
    loop {}
}
