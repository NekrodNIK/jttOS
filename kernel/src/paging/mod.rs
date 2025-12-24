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

pub fn init_paging_regs() {
    unsafe {
        asm!(
            "mov eax, cr0",
            "and eax, ~(1 << 16)",
            "mov cr0, eax",

            "mov eax, cr4",
            "or eax, 1 << 4",
            "mov cr4, eax",

            "mov cr3, {}",
            in(reg) PAGE_DIRECTORY
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

pub fn init_fb_paging() {
    let start_addr = (unsafe { crate::framebuffer_addr as usize } & (0x3ff * HUGE_PAGE_SIZE));
    let end_addr = {
        let addr = unsafe {
            crate::framebuffer_addr as usize
                + crate::framebuffer_width as usize * crate::framebuffer_height as usize
        };
        (addr + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1)
    };

    for i in (start_addr >> 22)..(end_addr >> 22) {
        unsafe {
            (*PAGE_DIRECTORY)[i] =
                PageDirectoryEntry::new_4mb((i * HUGE_PAGE_SIZE) as _, true, true, false)
        }
    }
}

pub fn init_kernel_paging() {
    let pt0 = unsafe {
        PAGE_DIRECTORY = POOL4K.alloc() as *mut [PageDirectoryEntry; 1024];
        *PAGE_DIRECTORY = array::from_fn(|_| PageDirectoryEntry::empty());
        (*PAGE_DIRECTORY)[0] = PageDirectoryEntry::new_4kb(POOL4K.alloc() as _, true, true, true);
        &mut *(*PAGE_DIRECTORY)[0].pt_addr()
    };

    *pt0 = array::from_fn(|i| {
        PageTableEntry::new(
            (i * PAGE_SIZE) as _,
            true,
            true,
            (..0x7000).contains(&(i * PAGE_SIZE)) || (0x80000..0x400000).contains(&(i * PAGE_SIZE)),
        )
    });

    init_fb_paging();
    init_paging_regs();
}

pub fn init_user_paging() {
    unsafe {
        let pde1 = &mut (*PAGE_DIRECTORY)[1];
        let pde2 = &mut (*PAGE_DIRECTORY)[2];

        if pde1.is_empty() {
            let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
            for i in 0..1024 {
                (*pt)[i] = PageTableEntry::new((POOL4K.alloc()) as _, i == 1024, true, true);
            }
            (*pde1) = PageDirectoryEntry::new_4kb(pt as _, true, true, true);
        } else if pde1.present() && !pde1.pt_addr().is_null() {
            for pt in (&mut *pde1.pt_addr()).iter_mut() {
                if !pt.is_empty() && pt.present() {
                    POOL4K.free(pt.page_addr() as _);
                    *pt = PageTableEntry::new((POOL4K.alloc()) as _, true, true, true);
                }
            }
        }

        if pde2.is_empty() {
            let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
            for i in 0..1024 {
                (*pt)[i] = PageTableEntry::new(
                    (0x20000 + i * PAGE_SIZE) as _,
                    (0..16).contains(&i),
                    true,
                    true,
                )
            }

            (*pde2) = PageDirectoryEntry::new_4kb(pt as _, true, true, true);
        }
    }
}

pub fn init_args_pages(user_args: &[&[u8]]) -> (u32, *const *const u8) {
    const START: usize = 0x810_000;

    debug_assert!(user_args.len() <= 1008);
    let argc = user_args.len();

    let pt2 = unsafe { &mut *(*PAGE_DIRECTORY)[2].pt_addr() };
    let argv_page = POOL4K.alloc() as *mut *mut u8;
    pt2[16] = PageTableEntry::new(argv_page as _, true, true, true);

    for (i, string) in user_args.iter().enumerate() {
        debug_assert!(string.len() <= PAGE_SIZE);
        let page = POOL4K.alloc();
        pt2[17 + i] = PageTableEntry::new(page as _, true, true, true);

        unsafe {
            page.copy_from(string.as_ptr(), string.len());
            *argv_page.add(i) = (START + (i + 1) * PAGE_SIZE) as _;
        }
    }

    (argc as _, START as _)
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
