use super::ports::{inb, outb};
use core::fmt;

pub struct Uart {
    port: u16,
}

impl Uart {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub unsafe fn init(&mut self) {
        unsafe {
            // Disable all interrupts
            outb(self.port + 1, 0x00);
            // Enable DLAB
            outb(self.port + 3, 0x80);
            // Set baudrate 115200
            outb(self.port + 0, 0x01); // low_byte
            outb(self.port + 1, 0x00); // high_byte
            // Disable DLAB and set mode 8N1
            outb(self.port + 3, 0x03);
            // Enable FIFO, clear TX/RX buf, with 14-byte threshold
            outb(self.port + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(self.port + 4, 0x0B);
            // Enable interrupts
            outb(self.port + 1, 0x01);
        }
    }

    fn send(&mut self, data: u8) {
        while self.try_send(data).is_err() {}
    }

    fn try_send(&mut self, data: u8) -> Result<(), ()> {
        let is_transmit_empty = unsafe { inb(self.port + 5) & 0x20 };

        if is_transmit_empty == 0 {
            return Err(());
        }

        unsafe {
            outb(self.port, data);
        };

        Ok(())
    }
}

impl fmt::Write for Uart {
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(self, args)
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut buf = [0; 4];
        c.encode_utf8(&mut buf);

        for b in buf.iter() {
            self.send(*b);
        }

        Ok(())
    }
}
