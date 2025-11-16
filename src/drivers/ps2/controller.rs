use super::super::port::Port;
use bitflags::bitflags;

const COMMAND: Port<u8> = Port::new(0x64);
const DATA: Port<u8> = Port::new(0x60);
const WRITE_COMMAND_BYTE: u8 = 0x60;

bitflags! {
    struct CommandByte : u8 {
        const FIRST_TRASLATION = 0b10000;
        const SECOND_CLOCK = 0b10000;
        const FIRST_CLOCK = 0b1000;
        const SYSTEM_FLAG = 0b100;
        const SECOND_INTERRUPT = 0b10;
        const FIRST_INTERRUPT = 0b1;
    }
}

pub struct PS2Controller;

impl PS2Controller {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self) {
        let cbyte = CommandByte::FIRST_INTERRUPT | CommandByte::FIRST_CLOCK;
        COMMAND.write(WRITE_COMMAND_BYTE);
        DATA.write(cbyte.bits());
    }
}
