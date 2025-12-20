#![no_std]
#![no_main]

use core::arch::asm;
use utils::{framebuffer, io::Write, key::Key, textbuffer};

fn get_fb() -> framebuffer::Framebuffer {
    let mut addr: *mut u32;
    let mut width: u32;
    let mut height: u32;
    unsafe {
        asm!("int 0x80",
            inout("eax") 10 => addr);
        asm!("int 0x80",
            inout("eax") 11 => width);
        asm!("int 0x80",
            inout("eax") 12 => height);
    }

    framebuffer::Framebuffer {
        addr,
        height: height as usize,
        width: width as usize,
    }
}

fn get_key() -> u8 {
    let mut code: i32 = 0;
    while code == 0 {
        unsafe {
            asm!("int 0x80",
                inout("eax") 3 => code,
                in("ebx") 0);
        }
    }
    code as u8
}

pub fn key_to_symbol(key: Key) -> Option<u8> {
    match key {
        Key::BackTick => Some(b'`'),
        Key::Key1 => Some(b'1'),
        Key::Key2 => Some(b'2'),
        Key::Key3 => Some(b'3'),
        Key::Key4 => Some(b'4'),
        Key::Key5 => Some(b'5'),
        Key::Key6 => Some(b'6'),
        Key::Key7 => Some(b'7'),
        Key::Key8 => Some(b'8'),
        Key::Key9 => Some(b'9'),
        Key::Key0 => Some(b'0'),
        Key::Minus => Some(b'-'),
        Key::Equal => Some(b'='),
        Key::Q => Some(b'q'),
        Key::W => Some(b'w'),
        Key::E => Some(b'e'),
        Key::R => Some(b'r'),
        Key::T => Some(b't'),
        Key::Y => Some(b'y'),
        Key::U => Some(b'u'),
        Key::I => Some(b'i'),
        Key::O => Some(b'o'),
        Key::P => Some(b'p'),
        Key::OpenBrace => Some(b'['),
        Key::CloseBrace => Some(b']'),
        Key::Backslash => Some(b'\\'),
        Key::A => Some(b'a'),
        Key::S => Some(b's'),
        Key::D => Some(b'd'),
        Key::F => Some(b'f'),
        Key::G => Some(b'g'),
        Key::H => Some(b'h'),
        Key::J => Some(b'j'),
        Key::K => Some(b'k'),
        Key::L => Some(b'l'),
        Key::SemiColon => Some(b';'),
        Key::SingleQuote => Some(b'\''),
        Key::Z => Some(b'z'),
        Key::X => Some(b'x'),
        Key::C => Some(b'c'),
        Key::V => Some(b'v'),
        Key::B => Some(b'b'),
        Key::N => Some(b'n'),
        Key::M => Some(b'm'),
        Key::Comma => Some(b','),
        Key::Dot => Some(b'.'),
        Key::Slash => Some(b'/'),
        Key::Space => Some(b' '),
        _ => None,
    }
}

pub fn key_to_upper_symbol(key: Key) -> Option<u8> {
    match key {
        Key::BackTick => Some(b'~'),
        Key::Key1 => Some(b'!'),
        Key::Key2 => Some(b'@'),
        Key::Key3 => Some(b'#'),
        Key::Key4 => Some(b'$'),
        Key::Key5 => Some(b'%'),
        Key::Key6 => Some(b'^'),
        Key::Key7 => Some(b'&'),
        Key::Key8 => Some(b'*'),
        Key::Key9 => Some(b'('),
        Key::Key0 => Some(b')'),
        Key::Minus => Some(b'_'),
        Key::Equal => Some(b'+'),
        Key::Q => Some(b'Q'),
        Key::W => Some(b'W'),
        Key::E => Some(b'E'),
        Key::R => Some(b'R'),
        Key::T => Some(b'T'),
        Key::Y => Some(b'Y'),
        Key::U => Some(b'U'),
        Key::I => Some(b'I'),
        Key::O => Some(b'O'),
        Key::P => Some(b'P'),
        Key::OpenBrace => Some(b'{'),
        Key::CloseBrace => Some(b'}'),
        Key::Backslash => Some(b'|'),
        Key::A => Some(b'A'),
        Key::S => Some(b'S'),
        Key::D => Some(b'D'),
        Key::F => Some(b'F'),
        Key::G => Some(b'G'),
        Key::H => Some(b'H'),
        Key::J => Some(b'J'),
        Key::K => Some(b'K'),
        Key::L => Some(b'L'),
        Key::SemiColon => Some(b':'),
        Key::SingleQuote => Some(b'"'),
        Key::Z => Some(b'Z'),
        Key::X => Some(b'X'),
        Key::C => Some(b'C'),
        Key::V => Some(b'V'),
        Key::B => Some(b'B'),
        Key::N => Some(b'N'),
        Key::M => Some(b'M'),
        Key::Comma => Some(b'<'),
        Key::Dot => Some(b'>'),
        Key::Slash => Some(b'?'),
        Key::Space => Some(b' '),
        _ => None,
    }
}

const SHELL_PROMPT: &'static str = "jttOS> ";

pub fn entry() {
    let fb = get_fb();
    let mut tbw = textbuffer::TextBufferWritter::new(textbuffer::TextBuffer::new(fb));

    tbw.clear();
    tbw.set_next_fg(0x000ff00);
    write!(tbw, "{}", SHELL_PROMPT).unwrap();
    tbw.set_next_fg(0x00ffffff);

    let mut upper_flag = false;
    let mut command_buf = [0; 5];
    let mut command_index = 0;
    let mut command_wait_flag = true;

    loop {
        let mut keycode: u8 = 0;
        while keycode == 0 {
            keycode = get_key();
        }
        let key = match Key::from_repr(keycode) {
            Some(key) => key,
            None => continue,
        };

        match key {
            Key::Backspace => {
                if tbw.x > SHELL_PROMPT.len() {
                    tbw.step_back()
                }
            }
            Key::Enter => {
                match (command_wait_flag, command_buf) {
                    (
                        false,
                        [
                            b'c' | b'C',
                            b'l' | b'L',
                            b'e' | b'E',
                            b'a' | b'A',
                            b'r' | b'R',
                        ],
                    ) => {
                        tbw.clear();
                    }
                    _ => write!(tbw, "\n").unwrap(),
                }
                command_index = 0;
                command_wait_flag = true;

                tbw.set_next_fg(0x000ff00);
                write!(tbw, "{}", SHELL_PROMPT).unwrap();
                tbw.set_next_fg(0x00ffffff);
            }
            _ => (),
        }

        let symfn = if upper_flag {
            key_to_upper_symbol
        } else {
            key_to_symbol
        };

        match symfn(key) {
            Some(symbol) => {
                if command_index < command_buf.len() - 1 {
                    command_buf[command_index] = symbol;
                    command_index += 1;
                } else if command_index == command_buf.len() - 1 {
                    command_buf[command_index] = symbol;
                    command_index += 0;
                    command_wait_flag = false;
                }
                write!(tbw, "{}", symbol as char).unwrap();
            }
            None => (),
        }

        if matches!(key, Key::CapsLock) {
            upper_flag = !upper_flag
        }
    }
}
