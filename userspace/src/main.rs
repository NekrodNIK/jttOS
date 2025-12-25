#![no_std]
#![no_main]

mod stdlib;

pub fn main(args: &[*const u8]) {
    println!("Hello user!\n");
    for s in args {
        let mut p = *s;
        unsafe {
            while *p != 0 {
                print!("{}", *p as char);
                p = p.add(1);
            }
        }
        println!("");
    }

    // for i in 0.. {
    //     print!("{} ", i);
    // }
}
