use crate::{
    PICS, console,
    interrupts::InterruptContext,
    io::Write,
    port::Port,
    ps2::key::{Key, KeyEvent},
    ps2::parser,
    ps2::parser::KeyParser,
};

pub struct Keyboard {
    parser: KeyParser,
}

impl Keyboard {
    const COMMAND: Port<u8> = Port::new(0x64);
    const DATA: Port<u8> = Port::new(0x60);

    pub const fn new() -> Self {
        Self {
            parser: KeyParser::new(),
        }
    }

    pub fn init(&self) {
        // FIXME: WAIT WAIT WAIT
        Self::COMMAND.write(0x20);
        let cfg = Self::DATA.read();

        Self::COMMAND.write(0x60);
        Self::DATA.write(cfg & !(1 << 6));

        Self::DATA.write(0xf0);
        Self::DATA.write(0x02);
    }

    pub fn int_handler(&mut self, ctx: &InterruptContext) {
        let scancode = Self::DATA.read();

        match self.parser.parse(scancode) {
            Ok(KeyEvent::Pressed(key)) => console::println!("Key pressed: {:?}", key),
            Ok(KeyEvent::Up(key)) => console::println!("Key up: {:?}", key),
            Err(parser::Error::Incomplete) => (),
            Err(parser::Error::Unrecognized) => {
                console::warning!("Keyboard: scancode not recognized")
            }
            Err(parser::Error::TooManyKeys) => console::warning!("Keyboard: too many keys"),
        };

        PICS.lock().send_eoi(1);
    }
}
