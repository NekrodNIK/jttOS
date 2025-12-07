use crate::{
    console,
    drivers::{pic8259, pit, ps2},
    io::Write,
};

pub static DEVICES: DeviceManager = DeviceManager::new();

pub struct DeviceManager {
    pub pic: pic8259::ChainedPics,
    pub ps2controller: ps2::PS2Controller,
    pub pit: pit::Pit,
}

impl DeviceManager {
    pub const fn new() -> Self {
        Self {
            pic: pic8259::ChainedPics::new(0x20, 0x28),
            ps2controller: ps2::PS2Controller::new(),
            pit: pit::Pit::new(),
        }
    }

    pub fn init_devices(&self) {
        console::println!(console::yellow!("{:=^80}"), "DEVICES");

        self.pic.init(true);
        console::info!("PICs initializated");

        self.ps2controller.init();
        console::info!("PS2 controller initializated");

        self.pit.init(20);
        console::info!("PIT initializated");

        console::println!(console::yellow!("{:=<80}"), "");
    }
}
