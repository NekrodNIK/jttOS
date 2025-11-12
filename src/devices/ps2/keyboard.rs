use crate::{
    PICS, console,
    devices::{
        device::Device,
        ps2::parser::{Error, KeyParser},
    },
    interrupts::InterruptContext,
    io::Write,
    port::Port,
};

pub struct PS2Keyboard {
    parser: KeyParser,
}

impl Device for PS2Keyboard {
    fn init(&self) {
        // FIXME: WAIT WAIT WAIT
        // Self::DATA.write(0xf0);
        // Self::DATA.write(0x02);
    }
}

impl PS2Keyboard {
    const DATA: Port<u8> = Port::new(0x60);

    pub const fn new() -> Self {
        Self {
            parser: KeyParser::new(),
        }
    }

    pub fn int_handler(&mut self, ctx: &InterruptContext) {
        let scancode = Self::DATA.read();

        match self.parser.parse(scancode) {
            Ok(event) => {
                console::println!("{:?}", event);
            }
            Err(Error::Incomplete) => (),
            Err(Error::Unrecognized) => {
                console::warning!("Keyboard: scancode not recognized")
            }
            Err(Error::TooManyKeys) => console::warning!("Keyboard: too many keys"),
        };
    }
}
