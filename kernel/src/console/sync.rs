// FIXME: SHOULD BE REMOVED AFTER LAB6
use core::cell::UnsafeCell;

pub struct NullLock<T> {
    data: UnsafeCell<T>,
}

impl<T> NullLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            data: UnsafeCell::new(t),
        }
    }

    pub fn get(&self) -> *mut T {
        self.data.get()
    }
}

unsafe impl<T> Sync for NullLock<T> {}
unsafe impl<T> Send for NullLock<T> {}
