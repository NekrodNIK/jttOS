use core::cell::Cell;

use super::key::{Key, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserState {
    Normal,
    Extended,
    UpNormal,
    UpExtended,
    PauseKey(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Incomplete,
    Unrecognized,
    TooManyKeys,
}

pub struct KeyParser {
    state: Cell<ParserState>,
}

unsafe impl Sync for KeyParser {}
unsafe impl Send for KeyParser {}

impl KeyParser {
    pub const fn new() -> Self {
        Self {
            state: Cell::new(ParserState::Normal),
        }
    }

    pub fn parse(&self, scancode: u8) -> Result<KeyEvent, Error> {
        // TODO: little refactoring
        if scancode == 0x00 {
            return Err(Error::TooManyKeys);
        }

        let next_state = match (self.state.get(), scancode) {
            // Pause Key
            (ParserState::Normal, 0xE1) => ParserState::PauseKey(0),
            (ParserState::PauseKey(5), _) => ParserState::Extended,
            (ParserState::PauseKey(i), _) => ParserState::PauseKey(i + 1),

            (ParserState::Normal, 0xE0) => ParserState::Extended,
            (ParserState::Normal, 0xF0) => ParserState::UpNormal,
            (ParserState::Extended, 0xF0) => ParserState::UpExtended,
            (_, _) => ParserState::Normal,
        };

        match next_state {
            ParserState::Normal => (),
            _ => {
                self.state.set(next_state);
                return Err(Error::Incomplete);
            }
        }

        match self.state.get() {
            ParserState::Extended => {
                if scancode == 0x12 {
                    return Err(Error::Incomplete);
                }
            }
            ParserState::UpExtended => {
                if scancode == 0x7C {
                    return Err(Error::Incomplete);
                }
            }
            _ => (),
        }

        let key = match self.state.get() {
            ParserState::Normal | ParserState::UpNormal => Self::map_normal(scancode),
            ParserState::Extended | ParserState::UpExtended => Self::map_extended(scancode),
            _ => Some(Key::F1),
        };

        if key == None {
            return Err(Error::Unrecognized);
        }
        let key = key.unwrap();

        let event = match self.state.get() {
            ParserState::Normal | ParserState::Extended => KeyEvent::Pressed(key),
            ParserState::UpNormal | ParserState::UpExtended => KeyEvent::Up(key),
            _ => KeyEvent::Pressed(key),
        };

        self.state.set(ParserState::Normal);
        Ok(event)
    }

    fn map_normal(scancode: u8) -> Option<Key> {
        match scancode {
            0x76 => Some(Key::Esc),
            0x05 => Some(Key::F1),
            0x06 => Some(Key::F2),
            0x04 => Some(Key::F3),
            0x0C => Some(Key::F4),
            0x03 => Some(Key::F5),
            0x0B => Some(Key::F6),
            0x83 => Some(Key::F7),
            0x0A => Some(Key::F8),
            0x01 => Some(Key::F9),
            0x09 => Some(Key::F10),
            0x78 => Some(Key::F11),
            0x07 => Some(Key::F12),
            0x0E => Some(Key::BackTick),
            0x16 => Some(Key::Key1),
            0x1E => Some(Key::Key2),
            0x26 => Some(Key::Key3),
            0x25 => Some(Key::Key4),
            0x2E => Some(Key::Key5),
            0x36 => Some(Key::Key6),
            0x3D => Some(Key::Key7),
            0x3E => Some(Key::Key8),
            0x46 => Some(Key::Key9),
            0x45 => Some(Key::Key0),
            0x4E => Some(Key::Minus),
            0x55 => Some(Key::Equal),
            0x66 => Some(Key::Backspace),
            0x0D => Some(Key::Tab),
            0x15 => Some(Key::Q),
            0x1D => Some(Key::W),
            0x24 => Some(Key::E),
            0x2D => Some(Key::R),
            0x2C => Some(Key::T),
            0x35 => Some(Key::Y),
            0x3C => Some(Key::U),
            0x43 => Some(Key::I),
            0x44 => Some(Key::O),
            0x4D => Some(Key::P),
            0x54 => Some(Key::OpenBracket),
            0x5B => Some(Key::CloseBracket),
            0x5D => Some(Key::Backslash),
            0x58 => Some(Key::CapsLock),
            0x1C => Some(Key::A),
            0x1B => Some(Key::S),
            0x23 => Some(Key::D),
            0x2B => Some(Key::F),
            0x34 => Some(Key::G),
            0x33 => Some(Key::H),
            0x3B => Some(Key::J),
            0x42 => Some(Key::K),
            0x4B => Some(Key::L),
            0x4C => Some(Key::SemiColon),
            0x52 => Some(Key::SingleQuote),
            0x5A => Some(Key::Enter),
            0x12 => Some(Key::LeftShift),
            0x1A => Some(Key::Z),
            0x22 => Some(Key::X),
            0x21 => Some(Key::C),
            0x2A => Some(Key::V),
            0x32 => Some(Key::B),
            0x31 => Some(Key::N),
            0x3A => Some(Key::M),
            0x41 => Some(Key::Comma),
            0x49 => Some(Key::Dot),
            0x4A => Some(Key::Slash),
            0x59 => Some(Key::RightShift),
            0x14 => Some(Key::LeftCtrl),
            0x11 => Some(Key::LeftAlt),
            0x29 => Some(Key::Space),
            0x7E => Some(Key::ScrollLock),
            0x77 => Some(Key::NumpadLock),
            0x7B => Some(Key::NumpadMinus),
            0x7C => Some(Key::NumpadStar),
            0x79 => Some(Key::NumpadPlus),
            0x71 => Some(Key::NumpadDot),
            0x70 => Some(Key::Numpad0),
            0x69 => Some(Key::Numpad1),
            0x72 => Some(Key::Numpad2),
            0x7A => Some(Key::Numpad3),
            0x6B => Some(Key::Numpad4),
            0x73 => Some(Key::Numpad5),
            0x74 => Some(Key::Numpad6),
            0x6C => Some(Key::Numpad7),
            0x75 => Some(Key::Numpad8),
            0x7D => Some(Key::Numpad9),
            _ => None,
        }
    }

    fn map_extended(scancode: u8) -> Option<Key> {
        match scancode {
            0x11 => Some(Key::RightAlt),
            0x14 => Some(Key::RightCtrl),
            0x70 => Some(Key::Insert),
            0x6C => Some(Key::Home),
            0x7D => Some(Key::PageUp),
            0x71 => Some(Key::Delete),
            0x69 => Some(Key::End),
            0x7A => Some(Key::PageDown),
            0x75 => Some(Key::CursorUp),
            0x6B => Some(Key::CursorLeft),
            0x74 => Some(Key::CursorRight),
            0x72 => Some(Key::CursorDown),
            0x4A => Some(Key::NumpadSlash),
            0x5A => Some(Key::NumpadEnter),
            0x1F => Some(Key::LeftMeta),
            0x27 => Some(Key::RightMeta),
            0x7C => Some(Key::PrintScreen),
            0x12 => Some(Key::PrintScreen),
            0x77 => Some(Key::Pause),
            _ => None,
        }
    }
}
