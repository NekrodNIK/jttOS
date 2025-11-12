use core::{
    any::type_name,
    cell::{Cell, UnsafeCell},
    fmt::{self, Write},
    ops::{Deref, DerefMut},
};

use crate::utils::{EFlags, cli, sti};

pub struct IntSafe<T> {
    locked: Cell<bool>,
    saved_flag: Cell<bool>,
    data: UnsafeCell<T>,
}

impl<T> fmt::Display for IntSafe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}<{}>",
            type_name::<Self>(),
            type_name::<T>()
        ))
    }
}

impl<T> IntSafe<T> {
    pub const fn new(t: T) -> Self {
        Self {
            locked: Cell::new(false),
            saved_flag: Cell::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub unsafe fn get(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }

    pub fn lock(&self) -> IntSafeGuard<'_, T> {
        if let Some(guard) = self.try_lock() {
            guard
        } else {
            panic!("{}: double lock", self);
        }
    }

    pub fn try_lock(&self) -> Option<IntSafeGuard<'_, T>> {
        if self.locked.get() {
            return None;
        }

        self.saved_flag.set(EFlags::read().contains(EFlags::IF));

        if self.saved_flag.get() {
            unsafe { cli() }
        }

        self.locked.set(true);
        Some(IntSafeGuard::new(self))
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

unsafe impl<T> Sync for IntSafe<T> {}
unsafe impl<T> Send for IntSafe<T> {}

pub struct IntSafeGuard<'a, T> {
    lock: &'a IntSafe<T>,
}

impl<'a, T> IntSafeGuard<'a, T> {
    pub fn new(lock: &'a IntSafe<T>) -> Self {
        Self { lock }
    }
}

impl<T> Drop for IntSafeGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T> Deref for IntSafeGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for IntSafeGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: Default> Default for IntSafe<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
