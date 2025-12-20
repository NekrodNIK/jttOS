#![allow(dead_code)]

mod allocator;
mod tables;

use core::arch::asm;
use core::array;

use alloc::boxed::Box;
pub use tables::PageDirectoryEntry;
pub use tables::PageTableEntry;
use utils::io::Write;

use crate::paging::allocator::PAGE_ALLOCATOR;

pub const PAGE_SIZE: usize = 4 * 1024;

struct Page([u8; PAGE_SIZE]);

pub fn init_pagging_regs(pd: *const PageDirectoryEntry) {
    unsafe {
        asm!(
            "mov eax, cr0",
            "and eax, ~(1 << 16)",
            "mov cr0, eax",

            "mov eax, cr4",
            "or eax, 1 << 4",
            "mov cr4, eax",

            "mov cr3, {}",
            in(reg) pd
        )
    }
}

pub fn enable_paging() {
    unsafe { asm!("mov eax, cr0", "or eax, 1 << 31", "mov cr0, eax",) }
}

pub fn disable_paging() {
    unsafe { asm!("mov eax, cr0", "and eax, ~(1 << 31)", "mov cr0, eax",) }
}

pub fn init_paging<F: Fn(usize) -> bool>(us_f: F) {
    let f_dir = |i| {
        if us_f(i) {
            let pt = PAGE_ALLOCATOR.alloc() as *mut [PageTableEntry; 1024];
            unsafe {
                *pt = array::from_fn(|j| {
                    PageTableEntry::new(((i * 1024) + j) as u32, true, true, true)
                })
            }

            PageDirectoryEntry::new(pt as u32, true, true, true, false)
        } else {
            PageDirectoryEntry::empty()
        }
    };

    let pd = PAGE_ALLOCATOR.alloc() as *mut [PageDirectoryEntry; 1024];
    unsafe { *pd = array::from_fn(f_dir) };

    init_pagging_regs(pd as *const PageDirectoryEntry);
}
