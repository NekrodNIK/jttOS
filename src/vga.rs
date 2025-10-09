use crate::io;
use core::fmt;
use core::slice;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const ADDR: usize = 0xb8000;

#[derive(Debug)]
pub struct Vga {
    screen: &'static mut [Symbol],
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(packed)]
struct Symbol {
    code: u8,
    colors: u8,
}

impl Vga {
    pub const fn new() -> Self {
        let screen = unsafe { slice::from_raw_parts_mut(ADDR as *mut Symbol, WIDTH * HEIGHT) };
        Self { screen, x: 0, y: 0 }
    }

    pub fn clear(&mut self) {
        self.screen.fill(Symbol {
            code: 0,
            colors: 0xf,
        });
        self.x = 0;
        self.y = 0;
    }
}

impl io::Write for Vga {
    fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        for byte in buf {
            if *byte == b'\n' {
                self.x = 0;
                self.y = (self.y + 1) % HEIGHT;
                continue;
            }

            let mut s = Symbol::default();
            s.code = *byte;
            s.colors = 0xf;

            self.screen[self.y * WIDTH + self.x] = s;

            self.x += 1;
            self.y += self.x / WIDTH;
            self.x %= WIDTH;
            self.y %= HEIGHT;
        }

        Ok(())
    }
}
