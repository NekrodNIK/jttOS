#![allow(dead_code)]

mod allocator;

pub struct PhysAddr(usize);
pub struct VirtAddr(usize);

impl PhysAddr {
    pub const fn new(address: usize) -> Self {
        Self(address)
    }

    pub const fn data(&self) -> usize {
        self.0
    }
}

impl VirtAddr {
    pub const fn new(address: usize) -> Self {
        Self(address)
    }

    pub const fn data(&self) -> usize {
        self.0
    }
}

pub struct Page {}
