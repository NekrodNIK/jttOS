mod allocator;
mod entries;

use core::arch::asm;
use core::array;
use core::ptr;

pub use allocator::POOL4K;
pub use entries::PageDirectoryEntry;
pub use entries::PageTableEntry;
use utils::nullsync;

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
            } else if fb_start_pde_ind <= i && i < fb_end_pde_ind {
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
        if !pde.is_empty() {
            let p = if pde.is_huge() {
                pde.huge_page_addr() as _
            } else {
                for pt in (&*pde.pt_addr()).iter() {
                    POOL4K.free(pt.page_addr() as _);
                }
                pde.pt_addr() as _
            };
            POOL4K.free(p);
        }

        let user_pt = POOL4K.alloc() as *mut [PageTableEntry; 256];
        *user_pt = array::from_fn(|i| PageTableEntry::new((1 * 1024 + i) as _, true, true, true));
        (*pde) = PageDirectoryEntry::new_4kb(user_pt as _, true, true, true);
    }
}
