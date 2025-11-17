use crate::{
    critical_section,
    utils::{cli, sti},
};

use super::port::Port;
use bitflags::bitflags;

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

pub struct ChainedPics {
    master: Pic,
    slave: Pic,
}

struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
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

    fn wait(&self) {
        Port::<u8>::new(0x80).write(0);
    }

    pub fn init(&self, auto_eoi: bool) {
        let icw1 = ICW1::ICW4 | ICW1::INIT;
        self.master.command.write(icw1.bits());
        self.wait();
        self.slave.command.write(icw1.bits());
        self.wait();

        self.master.data.write(self.master.offset);
        self.wait();
        self.slave.data.write(self.slave.offset);
        self.wait();

        self.master.data.write(0b100);
        self.wait();
        self.slave.data.write(2);
        self.wait();

        let mut icw4 = ICW4::A0;
        if auto_eoi {
            icw4 |= ICW4::AUTO_EOI;
        }

        self.master.data.write(icw4.bits());
        self.wait();
        self.slave.data.write(icw4.bits());
        self.wait();

        self.master.data.write(0xff);
        self.wait();
        self.slave.data.write(0xff);
        self.wait();
    }

    pub fn enable_device(&self, mut irq: u8) {
        let port = if irq < 8 {
            &self.master.data
        } else {
            irq -= 8;
            &self.slave.data
        };

        port.update(|p| p & !(1 << irq));
    }

    pub fn disable_device(&self, mut irq: u8) {
        let port = if irq < 8 {
            &self.master.data
        } else {
            irq -= 8;
            &self.slave.data
        };

        port.update(|p| p | (1 << irq));
    }

    pub fn send_eoi(&self, irq: u8) {
        critical_section::wrap(|| {
            const EOI: u8 = 0x20;
            if irq >= 8 {
                self.slave.command.write(EOI);
            }
            self.master.command.write(EOI);
            self.wait();
        })
    }
}
