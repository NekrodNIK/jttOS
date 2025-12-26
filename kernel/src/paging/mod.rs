mod allocator;
mod entries;

use core::arch::asm;
use core::array;

pub use allocator::POOL4K;
pub use entries::PageDirectoryEntry;
pub use entries::PageTableEntry;

pub use entries::PageDirectory;

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
        )
    }
}

#[inline(always)]
pub fn enable_paging(pd: *mut PageDirectory) {
    unsafe { asm!("mov cr3, {}", "mov eax, cr0", "or eax, 1 << 31", "mov cr0, eax", in(reg) pd) }
}

#[inline(always)]
pub fn disable_paging() {
    unsafe {
        asm!("mov eax, cr0", "and eax, ~(1 << 31)", "mov cr0, eax");
    }
}

pub fn init_fb_paging(pd: *mut PageDirectory) {
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
            (*pd)[i] = PageDirectoryEntry::new_4mb((i * HUGE_PAGE_SIZE) as _, true, true, false)
        }
    }
}

pub fn init_kernel_paging() -> *mut PageDirectory {
    let pd = POOL4K.alloc() as *mut PageDirectory;

    let pt0 = unsafe {
        *pd = array::from_fn(|_| PageDirectoryEntry::empty());
        (*pd)[0] = PageDirectoryEntry::new_4kb(POOL4K.alloc() as _, true, true, true);
        &mut *(*pd)[0].pt_addr()
    };

    *pt0 = array::from_fn(|i| PageTableEntry::new((i * PAGE_SIZE) as _, true, true, false));

    init_fb_paging(pd);
    init_paging_regs();
    pd
}

pub fn init_code_pages(pd: *mut PageDirectory, phys: *mut u8) {
    unsafe {
        let pde2 = &mut (*pd)[2];
        if pde2.is_empty() {
            let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
            for i in 0..16 {
                (*pt)[i] =
                    PageTableEntry::new((phys as usize + i * PAGE_SIZE) as _, true, true, true)
            }

            for i in 16..1024 {
                (*pt)[i] = PageTableEntry::empty();
            }

            (*pde2) = PageDirectoryEntry::new_4kb(pt as _, true, true, true);
        }
    }
}

pub fn init_stack_pages(pd: *mut PageDirectory) {
    unsafe {
        let pde1 = &mut (*pd)[1];

        if pde1.is_empty() {
            let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
            for i in 0..1023 {
                (*pt)[i] = PageTableEntry::new(0 as _, false, true, true);
            }
            (*pt)[1023] = PageTableEntry::new((POOL4K.alloc()) as _, true, true, true);
            (*pde1) = PageDirectoryEntry::new_4kb(pt as _, true, true, true);
        } else if pde1.present() && !pde1.pt_addr().is_null() {
            for pt in (&mut *pde1.pt_addr()).iter_mut() {
                if !pt.is_empty() && pt.present() {
                    POOL4K.free(pt.page_addr() as _);
                    *pt = PageTableEntry::new((POOL4K.alloc()) as _, true, true, true);
                }
            }
        }
    }
}

pub fn delete_process_pages(pd: *mut PageDirectory) {
    let pt1 = unsafe { &mut *(*pd)[1].pt_addr() };
    let pt2 = unsafe { &mut *(*pd)[2].pt_addr() };

    for pte in pt1.iter() {
        if !pte.page_addr().is_null() && pte.present() {
            POOL4K.free(pte.page_addr() as _);
        }
    }
    POOL4K.free(&raw mut *pt1 as _);
    unsafe {
        (*pd)[1] = PageDirectoryEntry::empty();
    }

    for i in 16..1024 {
        if !pt2[i].page_addr().is_null() && pt2[i].present() {
            POOL4K.free(pt2[i].page_addr() as _);
        }
    }
}

pub fn init_args_pages(pd: *mut PageDirectory, user_args: &[&[u8]]) -> (u32, *const *const u8) {
    const START: usize = 0x810_000;

    debug_assert!(user_args.len() <= 1008);
    let argc = user_args.len();

    let pt2 = unsafe { &mut *(*pd)[2].pt_addr() };
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

static mut CUR_STACK_INDEX: usize = 1023;

pub fn enable_stack_pages(pd: *mut PageDirectory, address: u32) {
    let pt1 = unsafe { &mut *(*pd)[1].pt_addr() };
    let index = (address as usize >> 12) & 0x3ff;

    unsafe {
        for i in index..CUR_STACK_INDEX {
            pt1[i] = PageTableEntry::new(POOL4K.alloc() as _, true, pt1[i].rw(), pt1[i].us());
        }
        CUR_STACK_INDEX = index;
    }
}
