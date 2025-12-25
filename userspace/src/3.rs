#![no_std]
#![no_main]

mod stdlib;

pub fn main(_args: &[*const u8]) {
    let mut x = 0;
    loop {
        match (x % 3 == 0, x % 5 == 0) {
            (true, true) => print!("FizzBuzz "),
            (true, _) => print!("Fizz "),
            (_, true) => print!("Buzz "),
            (_, _) => print!("{} ", x),
        }

        x += 1;
    }
}
