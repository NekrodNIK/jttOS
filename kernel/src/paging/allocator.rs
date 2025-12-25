use core::ptr;
use utils::nullsync;

use crate::{print, println};

const ARENA_START: usize = 0x400000;
unsafe extern "C" {
    static RAM_SIZE: usize;
}

type Pool4K = PoolAllocator<4096>;

pub static POOL4K: nullsync::LazyCell<Pool4K> = nullsync::LazyCell::new(|| {
    Pool4K::new(
        ptr::null_mut(),
        ARENA_START as _,
        (unsafe { RAM_SIZE }) as _,
    )
});

pub struct PoolAllocator<const N: usize> {
    state: nullsync::RefCell<PoolAllocatorState>,
}

struct PoolAllocatorState {
    pub freed: *mut *mut u8,
    pub current: *mut u8,
    pub end: *mut u8,
}

impl<const N: usize> PoolAllocator<N> {
    pub const fn new(freed: *mut *mut u8, current: *mut u8, end: *mut u8) -> Self {
        Self {
            state: nullsync::RefCell::new(PoolAllocatorState {
                freed,
                current,
                end,
            }),
        }
    }

    pub fn alloc(&self) -> *mut u8 {
        let mut state = self.state.borrow_mut();

        unsafe {
            if !state.freed.is_null() {
                let res = state.freed;
                state.freed = *state.freed as *mut *mut u8;
                res as _
            } else {
                let res = state.current;
                state.current = state.current.byte_add(N);
                if state.current > state.end {
                    panic!(
                        "OOM: the arena is not enough to allocate\n\
                        chunk_size: {:x}\n\
                        arena_current: {:x?}\n\
                        arena_end: {:x?}",
                        N, state.current, state.end
                    );
                }
                res as _
            }
        }
    }

    pub fn free(&self, pointer: *mut u8) {
        let mut state = self.state.borrow_mut();
        let p = pointer as *mut *mut u8;

        unsafe {
            *p = state.freed as *mut u8;
            state.freed = p;
        }
    }
}
