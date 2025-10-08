use core::fmt;
use core::slice;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const ADDR: usize = 0xb8000;

#[derive(Debug)]
pub struct Vga<'a> {
    screen: &'a mut [Symbol],
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(packed)]
struct Symbol {
    code: u8,
    colors: u8,
}

impl<'a> Vga<'a> {
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

impl<'a> fmt::Write for Vga<'a> {
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(self, args)
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if c == '\n' {
            self.x = 0;
            self.y = (self.y + 1) % HEIGHT;
            return Ok(());
        }

        let mut s = Symbol::default();
        s.code = c as u8;
        s.colors = 0xf;

        self.screen[self.y * WIDTH + self.x] = s;

        self.x += 1;
        self.y += self.x / WIDTH;
        self.x %= WIDTH;
        self.y %= HEIGHT;

        Ok(())
    }
}
