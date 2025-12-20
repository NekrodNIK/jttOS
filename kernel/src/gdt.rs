use bitflags::bitflags;
use core::cell::Cell;
use core::mem;

pub static GDT: Gdt = Gdt::new();
#[unsafe(link_section = ".boot")]
#[unsafe(no_mangle)]
pub static GDT_DESC: GdtDescriptor = GdtDescriptor {
    size: (mem::size_of_val(&GDT) - 1) as u16,
    offset: &raw const GDT,
};

bitflags! {
    pub struct SDFlags0 : u8 {
        // CODE
        const CODE_WRITABLE = 1 << 1;
        const CODE_DIRECTION = 1 << 2;
        // DATA
        const DATA_READABLE = 1 << 1;
        const DATA_CONFORMING = 1 << 2;
        // SYSTEM
        const TSS_32BIT_AVAILABLE = 0b1001;
        const TSS_32BIT_BUSY = 0b1011;
        // COMMON
        const ACCESSED = 1 << 0;
        const EXECUTABLE = 1 << 3;
        const NON_SYSTEM = 1 << 4;
        const DPL0 = 0b00 << 5;
        const DPL1 = 0b01 << 5;
        const DPL2 = 0b10 << 5;
        const DPL3 = 0b11 << 5;
        const PRESENT = 0b1 << 7;
    }

    pub struct SDFlags1 : u8 {
        const L = 1 << 5;
        const DB = 1 << 6;
        const GRANULARITY = 1 << 7;
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SegmentDescriptor {
    pub limit0: u16,
    pub base0: u16,
    pub base1: u8,
    pub flags0: u8,
    pub flags1_limit1: u8,
    pub base2: u8,
}

#[repr(C, packed)]
pub struct Gdt {
    pub zero: SegmentDescriptor,
    pub kernel_code: Cell<SegmentDescriptor>,
    pub kernel_data: SegmentDescriptor,
    pub user_code: Cell<SegmentDescriptor>,
    pub user_data: SegmentDescriptor,
    pub tss: Cell<SegmentDescriptor>,
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    pub size: u16,
    pub offset: *const Gdt,
}

impl SegmentDescriptor {
    pub const fn zero() -> SegmentDescriptor {
        SegmentDescriptor {
            limit0: 0,
            base0: 0,
            base1: 0,
            flags0: 0,
            flags1_limit1: 0,
            base2: 0,
        }
    }

    const fn base_code() -> SegmentDescriptor {
        SegmentDescriptor {
            limit0: 0xff,
            base0: 0,
            base1: 0,
            flags0: SDFlags0::empty()
                .union(SDFlags0::CODE_WRITABLE)
                .union(SDFlags0::EXECUTABLE)
                .union(SDFlags0::NON_SYSTEM)
                .union(SDFlags0::PRESENT)
                .bits(),
            flags1_limit1: SDFlags1::from_bits_retain(0xff)
                .union(SDFlags1::DB)
                .union(SDFlags1::GRANULARITY)
                .bits(),
            base2: 0,
        }
    }

    const fn base_data() -> SegmentDescriptor {
        SegmentDescriptor {
            limit0: 0xff,
            base0: 0,
            base1: 0,
            flags0: SDFlags0::NON_SYSTEM
                .union(SDFlags0::DATA_READABLE)
                .union(SDFlags0::PRESENT)
                .bits(),
            flags1_limit1: SDFlags1::from_bits_retain(0xff)
                .union(SDFlags1::DB)
                .union(SDFlags1::GRANULARITY)
                .bits(),
            base2: 0,
        }
    }

    pub const fn kernel_code() -> SegmentDescriptor {
        Self::base_code()
    }

    pub const fn kernel_data() -> SegmentDescriptor {
        Self::base_data()
    }

    pub const fn user_code() -> SegmentDescriptor {
        let mut desc = Self::base_code();
        desc.flags0 |= SDFlags0::DPL3.bits();
        desc
    }

    pub const fn user_data() -> SegmentDescriptor {
        let mut desc = Self::base_data();
        desc.flags0 |= SDFlags0::DPL3.bits();
        desc
    }

    pub const fn tss() -> SegmentDescriptor {
        let mut desc = SegmentDescriptor::zero();
        desc.flags0 = SDFlags0::PRESENT
            .union(SDFlags0::TSS_32BIT_AVAILABLE)
            .bits();
        desc
    }

    pub const fn set_base(&mut self, base: u32) {
        self.base0 = (base & 0xffff) as u16;
        self.base1 = ((base >> 16) & 0xff) as u8;
        self.base2 = ((base >> 24) & 0xff) as u8;
    }

    pub const fn set_limit(&mut self, limit: u32) {
        debug_assert!(limit < (1 << 20));

        self.limit0 = (limit & 0xffff) as u16;
        self.flags1_limit1 &= 0xf0;
        self.flags1_limit1 |= ((limit >> 16) & 0xf) as u8;
    }
}

impl Gdt {
    pub const fn new() -> Gdt {
        Gdt {
            zero: SegmentDescriptor::zero(),
            kernel_code: Cell::new(SegmentDescriptor::kernel_code()),
            kernel_data: SegmentDescriptor::kernel_data(),
            user_code: Cell::new(SegmentDescriptor::user_code()),
            user_data: SegmentDescriptor::user_data(),
            tss: Cell::new(SegmentDescriptor::tss()),
        }
    }
}

unsafe impl Sync for Gdt {}

unsafe impl Sync for GdtDescriptor {}
