use bitflags::bitflags;

use crate::port::{Port, io_wait};

pub struct ChainedPics {
    master: Pic,
    slave: Pic,
}

struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

bitflags! {
    struct ICW1: u8 {
        const ICW4 = 0b1;
        const SINGLE_CASCADE = 0b10;
        const LEVEL_EDGE = 0b10000;
        const INIT   = 0b10000;
    }
}

bitflags! {
    struct ICW4: u8 {
        const AUTO_EOI = 0b10;
        const A0  = 0b01;
    }
}

impl ChainedPics {
    pub const fn new(offset1: u8, offset2: u8) -> Self {
        Self {
            master: Pic {
                offset: offset1,
                command: Port::new(0x20),
                data: Port::new(0x21),
            },
            slave: Pic {
                offset: offset2,
                command: Port::new(0xa0),
                data: Port::new(0xa1),
            },
        }
    }

    pub fn init(&self) {
        let icw1 = ICW1::ICW4 | !ICW1::SINGLE_CASCADE | !ICW1::LEVEL_EDGE | ICW1::INIT;
        self.master.command.write(icw1.bits());
        io_wait();
        self.slave.command.write(icw1.bits());
        io_wait();

        self.master.data.write(self.master.offset);
        io_wait();
        self.slave.data.write(self.slave.offset);
        io_wait();

        self.master.data.write(0b100);
        io_wait();
        self.slave.data.write(2);
        io_wait();

        let icw4 = ICW4::AUTO_EOI | ICW4::A0;
        self.master.data.write(icw4.bits());
        io_wait();
        self.slave.data.write(icw4.bits());
        io_wait();
    }
}
