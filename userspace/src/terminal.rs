use core::arch::asm;
use utils::{framebuffer, io::Write, key::Key, textbuffer};

pub fn rdtsc() -> u64 {
    let high: u32;
    let low: u32;

    unsafe {
        asm!("rdtsc", out("edx") high, out("eax") low);
    }
    (high as u64) << 32 | (low as u64)
}

pub fn tsc_sleep(ticks: u64) {
    let start = rdtsc();
    let mut cur = start;

    while cur - start < ticks {
        cur = rdtsc();
    }
}

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

fn get_key_nowait() -> u8 {
    let mut code: i32 = 0;
    unsafe {
        asm!("int 0x80",
                inout("eax") 3 => code,
                in("ebx") 0);
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
const COMMAND_CLEAR: &'static [u8] = b"clear";
const COMMAND_ANIMATION: &'static [u8] = b"color";

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
                match &command_buf[0..command_index] {
                    COMMAND_CLEAR => tbw.clear(),
                    COMMAND_ANIMATION => animation(&mut tbw),
                    _ => write!(tbw, "\n").unwrap(),
                }

                command_buf = [0; 5];
                command_index = 0;

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

        if let Some(symbol) = symfn(key) {
            if command_index < command_buf.len() {
                command_buf[command_index] = symbol;
                command_index += 1;
            }
            write!(tbw, "{}", symbol as char).unwrap();
        }

        if matches!(key, Key::CapsLock) {
            upper_flag = !upper_flag
        }
    }
}

fn animation(tbw: &mut textbuffer::TextBufferWritter) {
    let fb = &tbw.buffer.fb;

    let mut hue: f32 = 0.0;
    let saturation: f32 = 1.0;
    let value: f32 = 1.0;

    loop {
        let keycode = get_key_nowait();
        if keycode != 0 {
            break;
        }

        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r_prime, g_prime, b_prime) = match (hue as i32 / 60) % 6 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let r = ((r_prime + m) * 255.0) as u32;
        let g = ((g_prime + m) * 255.0) as u32;
        let b = ((b_prime + m) * 255.0) as u32;

        let color = (r << 16) | (g << 8) | b;

        unsafe {
            for i in 0..(fb.width * fb.height) {
                *fb.addr.add(i) = color;
            }
        }

        hue += 0.5;
        if hue >= 360.0 {
            hue = 0.0;
        }

        tsc_sleep(1000000);
    }

    tbw.clear();
}
