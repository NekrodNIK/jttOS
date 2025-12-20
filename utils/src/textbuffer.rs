use crate::{font, framebuffer::Framebuffer, io};

const WIDTH_SYMBOLS: usize = 80;
const HEIGHT_SYMBOLS: usize = 25;

pub struct TextBuffer {
    pub fb: Framebuffer,
}

pub struct TextBufferWritter {
    pub buffer: TextBuffer,
    pub x: usize,
    pub y: usize,
    pub fg: u32,
    pub bg: u32,
}

impl TextBuffer {
    pub fn new(fb: Framebuffer) -> Self {
        Self { fb }
    }

    pub fn clear(&self) {
        for i in 0..self.fb.width * self.fb.height {
            unsafe { self.fb.addr.add(i).write_volatile(0) }
        }
    }

    pub fn put(&self, x: usize, y: usize, ch: u8, fg: u32, bg: u32) {
        let font_index = ch as usize * 16;

        for row in 0..16 {
            let font_byte = font::FONT[font_index + row];

            for col in 0..8 {
                let x = x * 8 + col;
                let y = y * 16 + row;

                if x < self.fb.width && y < self.fb.height {
                    let pixel_index = y * self.fb.width + x;
                    let color = if font_byte & (1 << (7 - col)) != 0 {
                        fg
                    } else {
                        bg
                    };

                    unsafe {
                        *self.fb.addr.add(pixel_index) = color;
                    }
                }
            }
        }
    }

    pub fn scroll_down(&self) {
        let lines_to_copy = (HEIGHT_SYMBOLS - 1) * 16;
        for y in 0..lines_to_copy {
            for x in 0..self.fb.width {
                let src_index = (y + 16) * self.fb.width + x;
                let dst_index = y * self.fb.width + x;
                unsafe {
                    let pixel = self.fb.addr.add(src_index).read_volatile();
                    self.fb.addr.add(dst_index).write_volatile(pixel);
                }
            }
        }

        let last_row_start = (HEIGHT_SYMBOLS - 1) * 16 * self.fb.width;
        for i in 0..(16 * self.fb.width) {
            unsafe {
                self.fb.addr.add(last_row_start + i).write_volatile(0);
            }
        }
    }
}

impl TextBufferWritter {
    pub fn new(buffer: TextBuffer) -> Self {
        Self {
            buffer,
            x: 0,
            y: 0,
            fg: 0x00ffffff,
            bg: 0,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.x = 0;
        self.y = 0;
    }

    pub fn set_next_fg(&mut self, fg: u32) {
        self.fg = fg
    }

    pub fn set_next_bg(&mut self, bg: u32) {
        self.fg = bg
    }

    pub fn step_back(&mut self) {
        if self.x == 0 {
            if self.y == 0 {
                return;
            }
            self.y -= 1;
            self.x = WIDTH_SYMBOLS - 1;
        } else {
            self.x -= 1;
        }
        self.buffer.put(self.x, self.y, b' ', self.fg, self.bg);
    }
}

impl io::Write for TextBufferWritter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if self.y == HEIGHT_SYMBOLS {
            self.buffer.scroll_down();
            self.y -= 1;
        }

        let mut iter = buf.iter();
        while let Some(byte) = iter.next() {
            if *byte == b'\n' {
                self.x = 0;
                self.y += 1;
            } else {
                self.buffer.put(self.x, self.y, *byte, self.fg, self.bg);
                self.x += 1;
                self.y += self.x / WIDTH_SYMBOLS;
                self.x %= WIDTH_SYMBOLS;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
