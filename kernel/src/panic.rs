use core::panic::PanicInfo;

use crate::{new_tbw, x86_utils::cli};
use utils::io::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { cli() }
    let mut tbw = new_tbw();
    tbw.clear();
    write!(tbw, "[").unwrap();
    tbw.set_next_fg(0x00ff0000);
    write!(tbw, "KERNEL PANIC").unwrap();
    tbw.set_next_fg(0x00ffffff);
    writeln!(tbw, "]\n{}", info.message()).unwrap();
    loop {}
}
