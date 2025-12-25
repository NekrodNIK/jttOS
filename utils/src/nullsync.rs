use ::core::cell;
use ::core::ops::Deref;

#[repr(transparent)]
struct Marker<T>(T);

#[repr(transparent)]
pub struct LazyCell<T> {
    data: Marker<cell::LazyCell<T>>,
}

#[repr(transparent)]
pub struct RefCell<T> {
    data: Marker<cell::RefCell<T>>,
}

unsafe impl<T> Sync for Marker<T> {}

impl<T> LazyCell<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            data: Marker(cell::LazyCell::new(f)),
        }
    }
}

impl<T> RefCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            data: Marker(cell::RefCell::new(value)),
        }
    }
}

impl<T> Deref for LazyCell<T> {
    type Target = ::core::cell::LazyCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

impl<T> Deref for RefCell<T> {
    type Target = ::core::cell::RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.data.0
    }
}

unsafe impl<T> Sync for RefCell<T> {}
