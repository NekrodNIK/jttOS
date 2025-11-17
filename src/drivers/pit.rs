use crate::drivers::port::Port;
use bitflags::bitflags;

pub const MIN_DIVISOR: u32 = 2;
pub const MAX_DIVISOR: u32 = 65536;

pub const MIN_FREQ: u32 = MAX_FREQ.div_ceil(MAX_DIVISOR);
pub const MAX_FREQ: u32 = 1193182;

const CH0: Port<u8> = Port::new(0x40);
const CONTROL: Port<u8> = Port::new(0x43);

bitflags! {
    pub struct ControlWord: u8 {
        const CHANNEL0 = 0b00 << 6;
        const CHANNEL1 = 0b01 << 6;
        const CHANNEL2 = 0b10 << 6;

        const LOW_BYTE = 0b01 << 4;
        const HIGH_BYTE = 0b10 << 4;

        const MODE_INT_TERMINAL_COUNT = 0b000 << 1;
        const MODE_HARDWARE_RETRIGGERABLE_ONE_SHOT = 0b001 << 1;
        const MODE_RATE_GENERATOR = 0b010 << 1;
        const MODE_SQUARE_WAVE_GENERATOR = 0b011 << 1;
        const MODE_SOFTWARE_TRIGGERED_STROBE = 0b100 << 1;
        const MODE_HARDWARE_TRIGGERED_STROBE = 0b101 << 1;

        const BINARY = 0b0;
        const BCD = 0b1;
    }
}

pub struct Pit;

impl Pit {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, frequency: u32) {
        debug_assert!((MIN_FREQ..=MAX_FREQ).contains(&frequency));

        let cw = ControlWord::CHANNEL0
            | ControlWord::LOW_BYTE
            | ControlWord::HIGH_BYTE
            | ControlWord::MODE_RATE_GENERATOR
            | ControlWord::BINARY;
        CONTROL.write(cw.bits());

        self.set_divisor(MAX_FREQ / frequency);
    }

    fn set_divisor(&self, divisor: u32) {
        debug_assert!((MIN_DIVISOR..=MAX_DIVISOR).contains(&divisor));

        let divisor = (divisor % MAX_DIVISOR) as u16;
        CH0.write((divisor & 0xff) as _);
        CH0.write((divisor >> 8 & 0xff) as _);
    }
}
