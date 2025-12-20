use core::array;
use utils::io::Write;

use crate::{
    interrupts::Idt,
    jump_to_userspace,
    paging::{
        PAGE_SIZE, POOL_ALLOCATOR, PageDirectoryEntry, PageTableEntry, enable_paging, init_paging,
    },
};

pub fn run() {
    if cfg!(ex1) {
        ex1();
    } else if cfg!(ex2) {
        ex2();
    } else if cfg!(ex3) {
        ex3();
    } else if cfg!(ex4) {
        ex4();
    } else if cfg!(ex5) {
        ex5();
    } else if cfg!(ex6) {
        ex6();
    } else if cfg!(ex7) {
        ex7();
    }
    enable_paging();
}

pub fn ex1() {
    init_paging(PageDirectoryEntry::new_4mb(0 as _, true, true, true), false);
}

pub fn ex2() {
    init_paging(
        PageDirectoryEntry::new_4mb(0 as _, true, true, false),
        false,
    );
}

pub fn ex3() {
    let pt = POOL_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
    unsafe { *pt = array::from_fn(|i| PageTableEntry::new(i as _, true, true, true)) }
    init_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
}

pub fn ex4() {
    let pt = POOL_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
    unsafe { *pt = array::from_fn(|i| PageTableEntry::new(i as _, true, true, true)) }
    init_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, false),
        false,
    );
}

pub fn ex5() {
    let pt = POOL_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
    unsafe { *pt = array::from_fn(|i| PageTableEntry::new(i as _, true, true, false)) }
    init_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
}

pub fn ex6() {
    let pt = POOL_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                i as _,
                true,
                true,
                !(0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x100000), // It doesn't matter, we don't use VGA
            )
        })
    }
    init_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false, // It matters
    );
}

pub fn ex7() {
    let pt = POOL_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                i as _,
                true,
                true,
                !(0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x100000), // It doesn't matter, we don't use VGA
            )
        })
    }
    init_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false, // It matters
    );
}
