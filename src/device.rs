use crate::io;

pub trait Device {
    fn init();
}

pub trait InputDevice: Device {
    fn writer() -> impl io::Write;
}

pub trait ControllerDevice<'a>: Device {
    fn devices() -> &'a impl Device;
}
