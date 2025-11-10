use core::alloc::{GlobalAlloc, Layout};

use crate::sync::IntSafe;

const ARENA_SIZE: usize = 100 * 1024 * 1024;

const ARENA_START: usize = 0x100000;
const ARENA_END: usize = ARENA_START + ARENA_SIZE;

struct LinearAllocator {
    inner: IntSafe<State>,
}

struct State {
    current: *mut u8,
    end: *mut u8,
}

unsafe impl GlobalAlloc for LinearAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut state = self.inner.lock();

        let mut current = state.current;
        let allocated;
        unsafe {
            current = current.add(current.align_offset(layout.align()));
            allocated = current;
            current = current.add(layout.size());
        }

        if current > state.end {
            panic!(
                "OOM: the arena is not enough to allocate the layout \n\n{:?}\narena_current: {:x?}\narena_end: {:x?}",
                layout, state.current, state.end
            )
        }

        state.current = current;
        allocated
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: LinearAllocator = LinearAllocator {
    inner: IntSafe::new(State {
        current: ARENA_START as *mut u8,
        end: ARENA_END as *mut u8,
    }),
};
