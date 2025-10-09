use core::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

use crate::utils::{EFlags, cli, sti};

// TODO: double lock protect (mark this as UB or panic)
pub struct IrqSafe<T> {
    lock_count: Cell<usize>,
    saved_flag: Cell<bool>,
    data: UnsafeCell<T>,
}

impl<T> IrqSafe<T> {
    pub const fn new(t: T) -> Self {
        Self {
            lock_count: Cell::new(0),
            saved_flag: Cell::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> IrqSafeGuard<'_, T> {
        if self.lock_count.get() == 0 || !self.saved_flag.get() {
            self.saved_flag.set(EFlags::read().contains(EFlags::IF));

            if self.saved_flag.get() {
                unsafe { cli() }
            }
        }

        self.lock_count.update(|x| x + 1);
        IrqSafeGuard::new(self)
    }

    pub fn unlock(&self) {
        self.lock_count.update(|x| x.saturating_sub(1));
        if self.lock_count.get() == 0 {
            panic!("IrqSafe: unlock without lock");
        }

        if self.lock_count.get() == 0 && self.saved_flag.get() {
            unsafe { sti() }
        }
    }
}

unsafe impl<T> Sync for IrqSafe<T> {}
unsafe impl<T> Send for IrqSafe<T> {}

pub struct IrqSafeGuard<'a, T> {
    lock: &'a IrqSafe<T>,
}

impl<'a, T> IrqSafeGuard<'a, T> {
    pub fn new(lock: &'a IrqSafe<T>) -> Self {
        Self { lock }
    }
}

impl<T> Drop for IrqSafeGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T> Deref for IrqSafeGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return unsafe { &*self.lock.data.get() };
    }
}

impl<T> DerefMut for IrqSafeGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return unsafe { &mut *self.lock.data.get() };
    }
}

impl<T: Default> Default for IrqSafe<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
