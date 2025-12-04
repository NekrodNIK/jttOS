use crate::drivers::port;

pub const DEFAULT_COLORCODE: ColorCode = ColorCode::new(Color::White, Color::Black);

pub struct TextMode80x25 {
    buffer: *mut Character,
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct Character {
    pub code: u8,
    pub color: ColorCode,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ColorCode(u8);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl TextMode80x25 {
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    const ADDR: usize = 0xb8000;

    pub fn new() -> Self {
        Self {
            buffer: Self::ADDR as *mut Character,
        }
    }

    pub fn set_character(&mut self, character: Character, x: usize, y: usize) {
        unsafe {
            self.buffer
                .add(x + y * Self::WIDTH)
                .write_volatile(character)
        }
    }

    pub fn scroll_down(&mut self) {
        for y in 0..(Self::HEIGHT - 1) {
            for x in 0..Self::WIDTH {
                unsafe {
                    let src = self.buffer.add(x + (y + 1) * Self::WIDTH).read_volatile();
                    (self.buffer.add(x + y * Self::WIDTH)).write_volatile(src);
                }
            }
        }

        for x in 0..Self::WIDTH {
            unsafe {
                self.buffer
                    .add(x + (Self::HEIGHT - 1) * Self::WIDTH)
                    .write_volatile(Character::default())
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..Self::SIZE {
            unsafe { self.buffer.add(i).write_volatile(Character::default()) }
        }
    }

    pub fn disable_cursor(&mut self) {
        unsafe {
            port::outb(0x3d4, 0x0a);
            port::outb(0x3d5, 0x20);
        }
    }
}

impl Character {
    pub fn new(code: u8, color: ColorCode) -> Self {
        Self { code, color }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character::new(0, DEFAULT_COLORCODE)
    }
}

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | fg as u8)
    }

    pub fn set_fg(&mut self, fg: Color) {
        self.0 &= !0xf;
        self.0 |= fg as u8;
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.0 &= !0xf0;
        self.0 |= (bg as u8) << 4
    }

    pub fn get_fg(&self) -> Color {
        Color::from_u8(self.0 & 0xf).unwrap()
    }

    pub fn get_bg(&self) -> Color {
        Color::from_u8(self.0 & 0xf0).unwrap()
    }
}

impl Color {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Black),
            1 => Some(Self::Blue),
            2 => Some(Self::Green),
            3 => Some(Self::Cyan),
            4 => Some(Self::Red),
            5 => Some(Self::Magenta),
            6 => Some(Self::Brown),
            7 => Some(Self::LightGrey),
            8 => Some(Self::DarkGrey),
            9 => Some(Self::LightBlue),
            10 => Some(Self::LightGreen),
            11 => Some(Self::LightCyan),
            12 => Some(Self::LightRed),
            13 => Some(Self::Pink),
            14 => Some(Self::Yellow),
            15 => Some(Self::White),
            _ => None,
        }
    }
}
