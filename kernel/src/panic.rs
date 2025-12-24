use core::panic::PanicInfo;

use crate::{TBW, x86_utils::cli};
use utils::io::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cli();
    let mut tbw = TBW.borrow_mut();
    tbw.clear();
    write!(tbw, "[").unwrap();
    tbw.set_next_fg(0x00ff0000);
    write!(tbw, "KERNEL PANIC").unwrap();
    tbw.set_next_fg(0x00ffffff);
    writeln!(tbw, "]\n{}", info.message()).unwrap();
    loop {}
}
