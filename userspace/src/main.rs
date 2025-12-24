#![no_std]
#![no_main]

mod stdlib;

pub fn main(args: &[&str]) {
    println!("Hello user! argc: {}, argv: {:?}", args.len(), args);
    loop {}
}
