use core::ops::{Deref, DerefMut};

pub struct RefCell<T> {
    inner: ::core::cell::RefCell<T>,
}

impl<T> RefCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: ::core::cell::RefCell::new(value),
        }
    }
}

impl<T> Deref for RefCell<T> {
    type Target = ::core::cell::RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl<T> Sync for RefCell<T> {}
