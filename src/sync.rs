use core::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

use crate::utils::{EFlags, cli, sti};

pub struct IrqSafe<T> {
    locked: Cell<bool>,
    saved_flag: Cell<bool>,
    data: UnsafeCell<T>,
}

impl<T> IrqSafe<T> {
    pub const fn new(t: T) -> Self {
        Self {
            locked: Cell::new(false),
            saved_flag: Cell::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> IrqSafeGuard<'_, T> {
        if let Some(guard) = self.try_lock() {
            guard
        } else {
            panic!("IrqSafe: double lock");
        }
    }

    pub fn try_lock(&self) -> Option<IrqSafeGuard<'_, T>> {
        if self.locked.get() {
            return None;
        }

        self.saved_flag.set(EFlags::read().contains(EFlags::IF));

        if self.saved_flag.get() {
            unsafe { cli() }
        }

        self.locked.set(true);
        Some(IrqSafeGuard::new(self))
    }

    pub fn unlock(&self) {
        if self.try_unlock().is_none() {
            panic!("IrqSafe: unlock without lock");
        }
    }

    pub fn try_unlock(&self) -> Option<()> {
        if !self.locked.get() {
            return None;
        }

        self.locked.set(false);

        if self.saved_flag.get() {
            unsafe { sti() }
        }

        Some(())
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
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for IrqSafeGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: Default> Default for IrqSafe<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
