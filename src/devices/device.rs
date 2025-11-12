use alloc::vec::Vec;

pub trait Device {
    fn init(&self);
}

pub trait DeviceController: Device {
    fn devices(&self) -> Vec<&dyn Device>;
}
