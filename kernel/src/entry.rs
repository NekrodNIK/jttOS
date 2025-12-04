#[unsafe(no_mangle)]
extern "C" fn kentry() -> ! {
    crate::kmain();
    loop {}
}
