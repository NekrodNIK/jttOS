#![no_std]
#![no_main]

mod stdlib;

pub fn main(_args: &[*const u8]) {
    let mut x = 0;
    loop {
        print!("{} ", x);
        x += 1;
    }
}
