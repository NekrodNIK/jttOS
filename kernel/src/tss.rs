use core::arch::asm;
use core::mem;

pub static TSS: Tss = Tss::new(16, 0x7c00);

#[repr(C, align(1))]
pub struct Tss {
    _filler0: u32,
    pub esp0: u32,
    pub ss0: u16,
    _filler1: [u8; 92],
    pub iomap_base: u16,
    _filler2: u32,
}

impl Tss {
    pub const fn new(ss0: u16, esp0: u32) -> Tss {
        Tss {
            _filler0: 0,
            esp0,
            ss0,
            _filler1: [0; 92],
            iomap_base: mem::size_of::<Tss>() as u16,
            _filler2: 0,
        }
    }

    pub fn load(&self) {
        unsafe {
            asm!("ltr {0:x}", in(reg) 40);
        }
    }
}

unsafe impl Sync for Tss {}
