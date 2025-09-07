use core::arch::asm;

pub struct Idt(pub [IntDescriptor; 255]);

impl Idt {
    pub fn new() -> Self {
        return Self([IntDescriptor::default(); 255]);
    }

    pub unsafe fn load(&self) {
        unsafe {
            asm!("mov eax, {ptr}",
                 "lidt [eax]",
                 ptr = in(reg) self.0.as_ptr());
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(packed)]
pub struct IntDescriptor {
    offset_low: u16,
    selector: u16,
    zero: u8,
    attributes: u8,
    offset_high: u16,
}

impl IntDescriptor {
    pub fn set_entry(&mut self, entry: fn()) {
        let address = entry as u32;
        self.offset_low = (address & 0xffff) as u16;
        self.offset_high = (address >> 16) as u16;
        self.attributes = 0x8e;
    }
}
