use crate::io;
use core::slice;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const ADDR: usize = 0xb8000;

#[derive(Debug)]
pub struct Vga {
    screen: &'static mut [Symbol],
    x: usize,
    y: usize,
    fg: Color,
    bg: Color,
    default_fg: Color,
    default_bg: Color,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Color {
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

#[derive(Debug, Default, Clone, Copy)]
#[repr(packed)]
struct Symbol {
    code: u8,
    colors: u8,
}

impl Vga {
    pub const fn new() -> Self {
        let screen = unsafe { slice::from_raw_parts_mut(ADDR as *mut Symbol, WIDTH * HEIGHT) };
        Self {
            screen,
            x: 0,
            y: 0,
            fg: Color::White,
            bg: Color::Black,
            default_fg: Color::White,
            default_bg: Color::Black,
        }
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
        let mut iter = buf.iter();
        while let Some(byte) = iter.next() {
            if *byte == b'\n' {
                self.x = 0;
                self.y += 1;
            } else {
                // TODO: add more complete support ANSI escape
                // FIXME: improve readability
                if *byte == 0x1b {
                    if let Some(byte) = iter.next()
                        && *byte == b'['
                    {
                        if let (Some(byte1), Some(byte2), Some(byte3)) =
                            (iter.next(), iter.next(), iter.next())
                            && *byte3 == b'm'
                        {
                            let color_escape = &[*byte1, *byte2];

                            enum Command {
                                ColorFg(Color),
                                ColorBg(Color),
                                DefaultFg,
                                DefaultBg,
                                Skip,
                            }

                            let selected = match color_escape {
                                b"30" => Command::ColorFg(Color::Black),
                                b"31" => Command::ColorFg(Color::Red),
                                b"32" => Command::ColorFg(Color::Green),
                                b"33" => Command::ColorFg(Color::Yellow),
                                b"34" => Command::ColorFg(Color::Blue),
                                b"35" => Command::ColorFg(Color::Magenta),
                                b"36" => Command::ColorFg(Color::Cyan),
                                b"37" => Command::ColorFg(Color::White),
                                b"39" => Command::DefaultFg,

                                b"40" => Command::ColorBg(Color::Black),
                                b"41" => Command::ColorBg(Color::Red),
                                b"42" => Command::ColorBg(Color::Green),
                                b"43" => Command::ColorBg(Color::Yellow),
                                b"44" => Command::ColorBg(Color::Blue),
                                b"45" => Command::ColorBg(Color::Magenta),
                                b"46" => Command::ColorBg(Color::Cyan),
                                b"47" => Command::ColorBg(Color::White),
                                b"49" => Command::DefaultBg,
                                _ => Command::Skip,
                            };

                            match selected {
                                Command::ColorFg(color) => self.fg = color,
                                Command::ColorBg(color) => self.bg = color,
                                Command::DefaultFg => self.fg = self.default_fg,
                                Command::DefaultBg => self.bg = self.default_bg,
                                _ => (),
                            }
                        }
                    }
                    continue;
                }
                let mut s = Symbol::default();
                s.code = *byte;
                s.colors = self.fg as u8 | (self.bg as u8) << 4;

                self.screen[self.y * WIDTH + self.x] = s;

                self.x += 1;
                self.y += self.x / WIDTH;
                self.x %= WIDTH;
            }

            if self.y >= HEIGHT {
                self.screen.copy_within(WIDTH.., 0);
                self.screen[WIDTH * (HEIGHT - 1)..].fill(Symbol {
                    code: 0,
                    colors: 0xf,
                });
                self.y -= 1;
            }
        }

        Ok(())
    }
}
