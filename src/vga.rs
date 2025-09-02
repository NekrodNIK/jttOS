use core::fmt;
use core::slice;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

pub struct Vga<'a> {
    ptr: &'a mut [Symbol],
    pos: Position,
}

#[derive(Clone)]
#[repr(packed)]
struct Symbol {
    code: u8,
    colors: u8,
}

impl Symbol {
    pub fn new() -> Self {
        return Self { code: 0, colors: 0 };
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl<'a> Vga<'a> {
    pub fn new() -> Self {
        return Self {
            ptr: unsafe { slice::from_raw_parts_mut(0xb8000 as *mut Symbol, WIDTH * HEIGHT) },
            pos: Position { x: 0, y: 0 },
        };
    }

    pub fn clear(&mut self) {
        self.ptr.fill(Symbol {
            code: 0,
            colors: 0xf | 0x1 << 4,
        });
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
            self.pos.x = 0;
            self.pos.y = (self.pos.y + 1) % HEIGHT;
            return Ok(());
        }

        let mut s = Symbol::new();
        s.code = c as u8;
        s.colors = 0xf | 0x1 << 4;

        self.ptr[self.pos.y * WIDTH + self.pos.x] = s;

        self.pos.x += 1;
        self.pos.y += self.pos.x / WIDTH;
        self.pos.x %= WIDTH;
        self.pos.y %= HEIGHT;

        Ok(())
    }
}
// const Color = enum(u4) {
//     black = 0x0,
//     blue = 0x1,
//     green = 0x2,
//     cyan = 0x3,
//     red = 0x4,
//     magenta = 0x5,
//     brown = 0x6,
//     light_gray = 0x7,
//     dark_gray = 0x8,
//     light_blue = 0x9,
//     light_green = 0xa,
//     light_cyan = 0xb,
//     light_red = 0xc,
//     light_magenta = 0xd,
//     yellow = 0xe,
//     white = 0xf,
// };
