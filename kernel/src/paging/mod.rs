mod allocator;
mod entries;

use core::arch::asm;
use core::array;
use core::ptr;

pub use allocator::POOL4K;
pub use entries::PageDirectoryEntry;
pub use entries::PageTableEntry;

use crate::paging::entries::PageDirectory;

pub const PAGE_SIZE: usize = 4 * 1024;
pub const HUGE_PAGE_SIZE: usize = 4 * 1024 * 1024;
pub type Page = [u8; PAGE_SIZE];
pub type HugePage = [u8; HUGE_PAGE_SIZE];

pub fn init_paging_regs(pd: *mut PageDirectoryEntry) {
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

static mut PAGE_DIRECTORY: *mut PageDirectory = ptr::null_mut();

pub fn init_kernel_paging(kernel_pde: PageDirectoryEntry, fb_us: bool) {
    unsafe {
        let fb_start_pde_ind = (crate::framebuffer_addr as usize & (0x3ff << 22)) >> 22;
        let fb_end_pde_ind = {
            let addr = crate::framebuffer_addr as usize
                + crate::framebuffer_width as usize * crate::framebuffer_height as usize;
            let aligned = (addr + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1);
            aligned
        } >> 22;

        let f_dir = |i| {
            if i == 0 {
                kernel_pde.clone()
            } else if fb_start_pde_ind <= i && i <= fb_end_pde_ind {
                PageDirectoryEntry::new_4mb((i * HUGE_PAGE_SIZE) as _, true, true, fb_us)
            } else {
                PageDirectoryEntry::empty()
            }
        };

        PAGE_DIRECTORY = POOL4K.alloc() as *mut [PageDirectoryEntry; 1024];
        (*PAGE_DIRECTORY) = array::from_fn(f_dir);
        init_paging_regs(PAGE_DIRECTORY as _);
    }
}

pub fn init_user_paging() {
    unsafe {
        let pde = &mut (*PAGE_DIRECTORY)[1];
        if !pde.is_huge() && pde.present() {
            for pt in (&mut *pde.pt_addr()).iter_mut() {
                if !pt.is_empty() && pt.present() {
                    POOL4K.free(pt.page_addr() as _);
                    *pt = PageTableEntry::new((POOL4K.alloc()) as _, true, true, true);
                }
            }
        } else {
            let user_pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
            for i in 0..1024 {
                (*user_pt)[i] = PageTableEntry::new((POOL4K.alloc()) as _, i == 1024, true, true);
            }
            (*pde) = PageDirectoryEntry::new_4kb(user_pt as _, true, true, true);
        }
    }
}

pub fn enable_user_pages(address: *mut u8) {
    unsafe {
        let address = (address as u32) & (!0 << 12);
        for pte in &mut *(*PAGE_DIRECTORY)[1].pt_addr() {
            if pte.page_addr() >= address as _ {
                *pte = PageTableEntry::new(pte.page_addr() as _, true, pte.rw(), pte.us());
            }
        }
    }
}
