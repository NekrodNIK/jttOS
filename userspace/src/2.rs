#![no_std]
#![no_main]

mod stdlib;
use core::hint::black_box;

pub fn main(_args: &[*const u8]) {
    program2(0);
}

#[allow(unconditional_recursion)]
fn program2(total: usize) {
    println!("Stack usage: {} KB", total);
    let _ = black_box([0u8; 4 * 1024]);
    program2(total + 4)
}
