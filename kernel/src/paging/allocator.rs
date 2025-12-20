use core::ptr;
use utils::nullsync;

const CHUNK_SIZE: usize = 4 * 1024;

const ARENA_START: usize = 0x400000;
unsafe extern "C" {
    static RAM_SIZE: usize;
}

pub static POOL_ALLOCATOR: nullsync::LazyCell<PoolAllocator<CHUNK_SIZE>> =
    nullsync::LazyCell::new(|| {
        PoolAllocator::new(
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
        let prev_freed = state.freed as *mut u8;
        let prev_current = state.current;

        unsafe {
            if !prev_freed.is_null() {
                state.freed = *state.freed as *mut *mut u8;
                prev_freed
            } else {
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
                prev_current
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
