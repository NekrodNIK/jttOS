use core::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicUsize, Ordering},
};

const ARENA_SIZE: usize = 1024 * 1024 * 1024;
const ARENA_START: usize = 0x100000;
const ARENA_END: usize = ARENA_START + ARENA_SIZE;

pub struct LinearAllocator {
    cur: AtomicUsize,
}

unsafe impl GlobalAlloc for LinearAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            let cur = self.cur.load(Ordering::Acquire);
            let aligned = (cur + layout.align() - 1) & !(layout.align() - 1);
            let new = aligned + layout.size();

            if new > ARENA_END {
                panic!(
                    "OOM: the arena is not enough to allocate the layout\n\
                    {:?}\n\
                    arena_current: {:x?}\n\
                    arena_end: {:x?}",
                    layout, cur, ARENA_END
                );
            }

            if self
                .cur
                .compare_exchange_weak(cur, new, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return aligned as _;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: LinearAllocator = LinearAllocator {
    cur: AtomicUsize::new(ARENA_START),
};
