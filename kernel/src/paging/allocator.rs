use crate::paging::Page;
use core::{ops::Deref, ptr};
use utils::nullsync;

const ARENA_START: usize = 0x400000;
unsafe extern "C" {
    static RAM_SIZE: usize;
}
static PAGE_ALLOCATOR: nullsync::LazyCell<PoolAllocator<Page>> = nullsync::LazyCell::new(|| {
    PoolAllocator::new(
        ptr::null_mut(),
        ARENA_START as _,
        (ARENA_START + unsafe { RAM_SIZE }) as _,
    )
});

pub struct PageBox(*mut Page);

struct PoolAllocator<T> {
    state: nullsync::RefCell<PoolAllocatorState<T>>,
}

struct PoolAllocatorState<T> {
    pub freed: *mut *mut T,
    pub current: *mut T,
    pub end: *mut T,
}

impl PageBox {
    pub fn new(value: Page) -> Self {
        let p = PAGE_ALLOCATOR.alloc();
        unsafe { *p = value };
        Self(p)
    }
}

impl Deref for PageBox {
    type Target = Page;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0) }
    }
}

impl Drop for PageBox {
    fn drop(&mut self) {
        PAGE_ALLOCATOR.free(self.0);
    }
}

impl<T> PoolAllocator<T> {
    pub const fn new(freed: *mut *mut T, current: *mut T, end: *mut T) -> Self {
        Self {
            state: nullsync::RefCell::new(PoolAllocatorState {
                freed,
                current,
                end,
            }),
        }
    }

    pub fn alloc(&self) -> *mut T {
        let mut state = self.state.borrow_mut();
        let prev_freed = state.freed as *mut T;
        let prev_current = state.current;

        unsafe {
            if !prev_freed.is_null() {
                state.freed = *state.freed as *mut *mut T;
                prev_freed
            } else {
                state.current = state.current.add(1);
                if state.current > state.end {
                    panic!("OOM: the arena is not enough to allocate a page");
                }
                prev_current
            }
        }
    }

    pub fn free(&self, pointer: *mut T) {
        let mut state = self.state.borrow_mut();
        let p = pointer as *mut *mut T;

        unsafe {
            *p = state.freed as *mut T;
            state.freed = p;
        }
    }
}
