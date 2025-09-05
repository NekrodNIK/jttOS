use core::arch::asm;

pub struct Idt([IntDes; 255]);

impl Idt {
    pub const fn new() -> Self {
        return Self([IntDes::new(); 255]);
    }

    pub unsafe fn load(&self) {
        unsafe {
            asm!("mov eax, {ptr}",
                 "lidt [eax]",
                 ptr = in(reg) self.0.as_ptr());
        }
    }
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct IntDes {
    offset1: u16,
    selector: u16,
    reserved: u8,
    type_attributes: u8,
    offset2: u16,
}

impl IntDes {
    const fn new() -> Self {
        Self {
            offset1: 0,
            selector: 0,
            reserved: 0,
            type_attributes: 0,
            offset2: 0,
        }
    }
}
