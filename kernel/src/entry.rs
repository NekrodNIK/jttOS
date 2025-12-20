#[unsafe(no_mangle)]
pub extern "C" fn kentry() -> ! {
    crate::kmain();
    loop {}
}
