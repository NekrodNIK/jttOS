use crate::{drivers::ps2::KeyParser, interrupts::InterruptContext};
use utils::key::{Key, KeyEvent};

use super::super::port::Port;
use bitflags::bitflags;
use utils::{nullsync, ringbuf};

const COMMAND: Port<u8> = Port::new(0x64);
const DATA: Port<u8> = Port::new(0x60);

const WRITE_COMMAND_BYTE: u8 = 0x60;
const SET_SCANCODE_SET: u8 = 0xF0;

const ACK: u8 = 0xFA;

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

pub struct PS2Keyboard;

pub static BUFFER: nullsync::RefCell<ringbuf::Ringbuf<Key, 1024>> =
    nullsync::RefCell::new(ringbuf::Ringbuf::new());

static PARSER: KeyParser = KeyParser::new();

impl PS2Keyboard {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self) -> bool {
        let wait = || Port::<u8>::new(0x80).write(0);

        let cbyte = CommandByte::FIRST_INTERRUPT | CommandByte::FIRST_CLOCK;
        COMMAND.write(WRITE_COMMAND_BYTE);
        DATA.write(cbyte.bits());
        wait();

        DATA.write(SET_SCANCODE_SET);
        wait();
        DATA.write(2);
        wait();

        let res = DATA.read() == ACK;
        wait();
        res
    }

    pub fn int_handler(ctx: &mut InterruptContext) {
        let code = DATA.read();
        if let Ok(KeyEvent::Pressed(key)) = PARSER.parse(code) { BUFFER.borrow_mut().push(key) }
    }

    pub fn read(&self) -> u8 {
        match BUFFER.borrow_mut().pop() {
            Some(value) => value.discriminant(),
            None => 0,
        }
    }
}
