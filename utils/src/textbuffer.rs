use crate::{font, framebuffer::Framebuffer, io};

pub struct TextBufferRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct TextBuffer {
    pub fb: Framebuffer,
    pub region: TextBufferRegion,
}

pub struct TextBufferWritter {
    pub buffer: TextBuffer,
    pub x: usize,
    pub y: usize,
    pub fg: u32,
    pub bg: u32,
}

impl TextBufferRegion {
    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.x <= x && self.y <= y && x < self.x + self.width && y < self.y + self.height
    }
}

impl TextBuffer {
    pub fn new(fb: Framebuffer) -> Self {
        let width = fb.width;
        let height = fb.height;

        Self {
            fb,
            region: TextBufferRegion {
                x: 0,
                y: 0,
                width: width,
                height: height,
            },
        }
    }

    pub fn sub(&self, region: TextBufferRegion) -> Self {
        Self {
            fb: self.fb.clone(),
            region,
        }
    }

    pub fn clear(&self) {
        for y in 0..self.region.height {
            for x in 0..self.region.width {
                unsafe {
                    self.fb
                        .addr
                        .add((self.region.y + y) * self.fb.width + (self.region.x + x))
                        .write_volatile(0)
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.region.width / 8
    }

    pub fn height(&self) -> usize {
        self.region.height / 16
    }

    pub fn put(&self, x: usize, y: usize, ch: u8, fg: u32, bg: u32) {
        let font_index = ch as usize * 16;

        for row in 0..16 {
            let font_byte = font::FONT[font_index + row];

            for col in 0..8 {
                let x = self.region.x + x * 8 + col;
                let y = self.region.y + y * 16 + row;

                if self.region.contains(x, y) {
                    let color = if font_byte & (1 << (7 - col)) != 0 {
                        fg
                    } else {
                        bg
                    };

                    unsafe {
                        *self.fb.addr.add(y * self.fb.width + x) = color;
                    }
                }
            }
        }
    }

    pub fn scroll_down(&self) {
        let lines_count = (self.height() - 1) * 16;
        for y in 0..lines_count {
            for x in 0..self.region.width {
                let src = (self.region.y + y + 16) * self.fb.width + (self.region.x + x);
                let dst = (self.region.y + y) * self.fb.width + (self.region.x + x);
                unsafe {
                    let pixel = self.fb.addr.add(src).read_volatile();
                    self.fb.addr.add(dst).write_volatile(pixel);
                }
            }
        }

        let last_row = (self.region.y + (self.height() - 1) * 16) * self.fb.width + self.region.x;
        for y in 0..16 {
            for x in 0..self.region.width {
                unsafe {
                    self.fb
                        .addr
                        .add(last_row + y * self.fb.width + x)
                        .write_volatile(0);
                }
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
        self.bg = bg
    }

    pub fn step_back(&mut self) {
        if self.x == 0 {
            if self.y == 0 {
                return;
            }
            self.y -= 1;
            self.x = self.buffer.width() - 1;
        } else {
            self.x -= 1;
        }
        self.buffer.put(self.x, self.y, b' ', self.fg, self.bg);
    }
}

impl io::Write for TextBufferWritter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if self.y == self.buffer.height() {
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
                self.y += self.x / self.buffer.width();
                self.x %= self.buffer.width();
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
