mod allocator;
mod entries;

use core::arch::asm;
use core::array;

pub use entries::PageDirectoryEntry;
pub use entries::PageTableEntry;

pub use crate::paging::allocator::POOL_ALLOCATOR;

pub const PAGE_SIZE: usize = 4 * 1024;
pub const HUGE_PAGE_SIZE: usize = 4 * 1024 * 1024;

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

pub fn init_paging(kernel_pde: PageDirectoryEntry, fb_us: bool) -> *mut [PageDirectoryEntry; 1024] {
    let fb_start_pde_ind = unsafe { crate::framebuffer_addr as usize & (0x3ff << 22) } >> 22;
    let fb_end_pde_ind = unsafe {
        let addr = crate::framebuffer_addr as usize
            + crate::framebuffer_width as usize * crate::framebuffer_height as usize;
        let aligned = (addr + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1);
        aligned
    } >> 22;

    let f_dir = |i| {
        if i == 0 {
            kernel_pde.clone()
        } else if fb_start_pde_ind <= i || i < fb_end_pde_ind {
            PageDirectoryEntry::new_4mb((i * HUGE_PAGE_SIZE) as _, true, true, fb_us)
        } else {
            PageDirectoryEntry::empty()
        }
    };

    let pd = POOL_ALLOCATOR.alloc() as *mut [PageDirectoryEntry; 1024];
    unsafe {
        *pd = array::from_fn(f_dir);
    }

    init_pagging_regs(pd as *const PageDirectoryEntry);
    pd
}
