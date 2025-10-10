use core::cell::LazyCell;

use crate::{io, irq::IrqSafe};

mod macros;
mod vga;

pub use macros::{clear, info, print, println};

pub static CONSOLE: IrqSafe<LazyCell<Console>> = IrqSafe::new(LazyCell::new(Console::new));

pub struct Console {
    output: vga::TextMode80x25,
    state: State,
}

struct State {
    pub x: usize,
    pub y: usize,
    pub fg: vga::Color,
    pub bg: vga::Color,
}

impl Console {
    pub fn new() -> Self {
        let mut new = Self {
            output: vga::TextMode80x25::new(),
            state: State {
                x: 0,
                y: 0,
                fg: vga::DEFAULT_COLORCODE.get_fg(),
                bg: vga::DEFAULT_COLORCODE.get_bg(),
            },
        };

        new.output.disable_cursor();
        new
    }

    pub fn clear(&mut self) {
        self.state.x = 0;
        self.state.y = 0;
        self.output.clear()
    }
}

impl io::Write for Console {
    fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        let mut iter = buf.iter();
        while let Some(byte) = iter.next() {
            if *byte == b'\n' {
                self.state.x = 0;
                self.state.y += 1;
            } else {
                // TODO: add more complete support ANSI escape codes
                // FIXME: improve readability
                if *byte == 0x1b {
                    if let Some(byte) = iter.next()
                        && *byte == b'['
                        && let (Some(byte1), Some(byte2), Some(byte3)) =
                            (iter.next(), iter.next(), iter.next())
                        && *byte3 == b'm'
                    {
                        let color_escape = &[*byte1, *byte2];

                        enum Command {
                            ColorFg(vga::Color),
                            ColorBg(vga::Color),
                            DefaultFg,
                            DefaultBg,
                            Skip,
                        }

                        let selected = match color_escape {
                            b"30" => Command::ColorFg(vga::Color::Black),
                            b"31" => Command::ColorFg(vga::Color::Red),
                            b"32" => Command::ColorFg(vga::Color::Green),
                            b"33" => Command::ColorFg(vga::Color::Yellow),
                            b"34" => Command::ColorFg(vga::Color::Blue),
                            b"35" => Command::ColorFg(vga::Color::Magenta),
                            b"36" => Command::ColorFg(vga::Color::Cyan),
                            b"37" => Command::ColorFg(vga::Color::White),
                            b"39" => Command::DefaultFg,

                            b"40" => Command::ColorBg(vga::Color::Black),
                            b"41" => Command::ColorBg(vga::Color::Red),
                            b"42" => Command::ColorBg(vga::Color::Green),
                            b"43" => Command::ColorBg(vga::Color::Yellow),
                            b"44" => Command::ColorBg(vga::Color::Blue),
                            b"45" => Command::ColorBg(vga::Color::Magenta),
                            b"46" => Command::ColorBg(vga::Color::Cyan),
                            b"47" => Command::ColorBg(vga::Color::White),
                            b"49" => Command::DefaultBg,
                            _ => Command::Skip,
                        };

                        match selected {
                            Command::ColorFg(color) => self.state.fg = color,
                            Command::ColorBg(color) => self.state.bg = color,
                            Command::DefaultFg => self.state.fg = vga::DEFAULT_COLORCODE.get_fg(),
                            Command::DefaultBg => self.state.bg = vga::DEFAULT_COLORCODE.get_bg(),
                            _ => (),
                        }
                    }
                    continue;
                }

                let character =
                    vga::Character::new(*byte, vga::ColorCode::new(self.state.fg, self.state.bg));
                self.output
                    .set_character(character, self.state.x, self.state.y);

                self.state.x += 1;
                self.state.y += self.state.x / vga::TextMode80x25::WIDTH;
                self.state.x %= vga::TextMode80x25::WIDTH;
            }

            if self.state.y >= vga::TextMode80x25::HEIGHT {
                self.output.scroll_down();
                self.state.y -= 1;
            }
        }

        Ok(())
    }
}
