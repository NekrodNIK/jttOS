use core::array;
use utils::io::Write;

use crate::{
    interrupts::Idt,
    jump_to_userspace,
    paging::{
        PAGE_SIZE, POOL4K, PageDirectoryEntry, PageTableEntry, enable_paging, enable_user_pages,
        init_kernel_paging, init_user_paging,
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
    } else if cfg!(ex8) {
        ex8();
    } else if cfg!(ex9) {
        ex9();
    } else if cfg!(ex10) {
        ex10();
    } else if cfg!(ex11) {
        ex11();
    }

    enable_paging();
}

pub fn ex1() {
    init_kernel_paging(PageDirectoryEntry::new_4mb(0 as _, true, true, true), false);
}

pub fn ex2() {
    init_kernel_paging(
        PageDirectoryEntry::new_4mb(0 as _, true, true, false),
        false,
    );
}

pub fn ex3() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe { *pt = array::from_fn(|i| PageTableEntry::new((i * PAGE_SIZE) as _, true, true, true)) }
    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
}

pub fn ex4() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe { *pt = array::from_fn(|i| PageTableEntry::new((i * PAGE_SIZE) as _, true, true, true)) }
    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, false),
        false,
    );
}

pub fn ex5() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| PageTableEntry::new((i * PAGE_SIZE) as _, true, true, false))
    }
    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
}

pub fn ex6() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x100000), // It doesn't matter, we don't use VGA
            )
        })
    }
    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false, // It matters
    );
}

pub fn ex7() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x100000), // It doesn't matter, we don't use VGA
            )
        })
    }
    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false, // It matters
    );
}

pub fn ex8() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(i * PAGE_SIZE <= 0x7000
                    || (0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x400_000)),
            )
        })
    }

    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
    init_user_paging();
}

pub fn ex9() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(i * PAGE_SIZE <= 0x7000
                    || (0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x400_000)),
            )
        })
    }

    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
    init_user_paging();
}

pub fn ex10() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(i * PAGE_SIZE <= 0x7000
                    || (0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x400_000)),
            )
        })
    }

    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
    init_user_paging();
    enable_user_pages(0x402_000 as _);
}

pub fn ex11() {
    let pt = POOL4K.alloc() as *mut [PageTableEntry; 1024];
    unsafe {
        *pt = array::from_fn(|i| {
            PageTableEntry::new(
                (i * PAGE_SIZE) as _,
                true,
                true,
                !(i * PAGE_SIZE <= 0x7000
                    || (0x80000 <= i * PAGE_SIZE && i * PAGE_SIZE <= 0x400_000)),
            )
        })
    }

    init_kernel_paging(
        PageDirectoryEntry::new_4kb(pt as _, true, true, true),
        false,
    );
    init_user_paging();
}
