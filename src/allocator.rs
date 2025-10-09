use core::alloc::{GlobalAlloc, Layout};

use crate::irq::IrqSafe;

const ARENA_SIZE: usize = 100 * 1024 * 1024;

const ARENA_START: usize = 0x100000;
const ARENA_END: usize = ARENA_START + ARENA_SIZE;

struct LinearAllocator {
    inner: IrqSafe<State>,
}

struct State {
    current: *mut u8,
    end: *mut u8,
}

unsafe impl GlobalAlloc for LinearAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut state = self.inner.lock();
        assert!(state.current < state.end);

        let allocated;

        unsafe {
            state.current = state
                .current
                .add(state.current.align_offset(layout.align()));
            allocated = state.current;
            state.current = state.current.add(layout.size());
        }

        assert!(state.current < state.end);
        allocated
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: LinearAllocator = LinearAllocator {
    inner: IrqSafe::new(State {
        current: ARENA_START as *mut u8,
        end: ARENA_END as *mut u8,
    }),
};
