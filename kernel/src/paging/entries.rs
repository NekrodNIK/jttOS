#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(pub u32);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageDirectoryEntry(pub u32);

impl PageTableEntry {
    pub const fn new(index: u32, present: bool, rw: bool, us: bool) -> Self {
        Self(index << 12 | (us as u32) << 2 | (rw as u32) << 1 | (present as u32))
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn frame_addr(&self) -> u32 {
        self.0 >> 12
    }

    pub fn present(&self) -> bool {
        self.0 & 0b1 != 0
    }

    pub fn rw(&self) -> bool {
        self.0 & 0b10 != 0
    }

    pub fn us(&self) -> bool {
        self.0 & 0b100 != 0
    }
}

impl PageDirectoryEntry {
    pub fn new_4kb(pt_addr: *const PageTableEntry, present: bool, rw: bool, us: bool) -> Self {
        Self(
            (pt_addr as u32) & (!0 << 12)
                | 0 << 7
                | (us as u32) << 2
                | (rw as u32) << 1
                | (present as u32),
        )
    }

    pub fn new_4mb(frame_addr: *const u8, present: bool, rw: bool, us: bool) -> Self {
        Self(
            (frame_addr as u32) & (!0 << 22)
                | 1 << 7
                | (us as u32) << 2
                | (rw as u32) << 1
                | (present as u32),
        )
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn table_addr(&self) -> u32 {
        self.0
    }

    pub fn present(&self) -> bool {
        self.0 & 0b1 != 0
    }

    pub fn rw(&self) -> bool {
        self.0 & 0b10 != 0
    }

    pub fn us(&self) -> bool {
        self.0 & 0b100 != 0
    }

    pub fn huge(&self) -> bool {
        self.0 & (1 << 7) != 0
    }
}
