#![no_std]
#![no_main]

mod stdlib;

pub fn main(_args: &[*const u8]) {
    println!("Hello user!");
}
