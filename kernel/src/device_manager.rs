use crate::{
    drivers::{pic8259, pit, ps2},
    interrupts,
};
use utils::io::Write;

pub static DEVICES: DeviceManager = DeviceManager::new();

pub struct DeviceManager {
    pub pic: pic8259::ChainedPics,
    pub ps2keyboard: ps2::PS2Keyboard,
    pub pit: pit::Pit,
}

impl DeviceManager {
    pub const fn new() -> Self {
        Self {
            pic: pic8259::ChainedPics::new(0x20, 0x28),
            ps2keyboard: ps2::PS2Keyboard::new(),
            pit: pit::Pit::new(),
        }
    }

    pub fn init_devices(&self, writter: &mut utils::textbuffer::TextBufferWritter) {
        writter.set_next_fg(0x00ffff00);
        writeln!(writter, "{:=^80}", "DEVICES").unwrap();
        writter.set_next_fg(0x00ffffff);

        self.pic.init(true);
        crate::info!(writter, "PICs initializated");

        self.ps2keyboard.init();
        interrupts::register_handler(0x21, ps2::PS2Keyboard::int_handler);
        self.pic.enable_device(1);
        crate::info!(writter, "PS2 controller initializated");

        self.pit.init(20);
        crate::info!(writter, "PIT initializated");

        writter.set_next_fg(0x00ffff00);
        writeln!(writter, "{:=^80}", "").unwrap();
        writter.set_next_fg(0x00ffffff);
    }
}
