use crate::paging::{HugePage, Page};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageTableEntry(pub u32);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageDirectoryEntry(pub u32);

pub type PageTable = [PageTableEntry; 1024];
pub type PageDirectory = [PageDirectoryEntry; 1024];

impl PageTableEntry {
    pub fn new(page: *mut Page, present: bool, rw: bool, us: bool) -> Self {
        debug_assert!(page as u32 & (1 << 13 - 1) == 0);
        Self(page as u32 & (!0 << 12) | (us as u32) << 2 | (rw as u32) << 1 | (present as u32))
    }

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn page_addr(&self) -> *mut Page {
        (self.0 & (!0 << 12)) as _
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
    pub fn new_4kb(pt: *mut PageTable, present: bool, rw: bool, us: bool) -> Self {
        debug_assert!(pt as u32 & (1 << 13 - 1) == 0);
        Self(
            (pt as u32) & (!0 << 12)
                | 0 << 7
                | (us as u32) << 2
                | (rw as u32) << 1
                | (present as u32),
        )
    }

    pub fn new_4mb(page: *mut HugePage, present: bool, rw: bool, us: bool) -> Self {
        debug_assert!(page as u32 & (1 << 23 - 1) == 0);
        Self(
            (page as u32) & (!0 << 22)
                | 1 << 7
                | (us as u32) << 2
                | (rw as u32) << 1
                | (present as u32),
        )
    }

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn pt_addr(&self) -> *mut PageTable {
        (self.0 & (!0 << 12)) as _
    }

    pub fn is_huge(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    pub fn huge_page_addr(&self) -> *mut PageTable {
        (self.0 & (!0 << 12)) as _
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
