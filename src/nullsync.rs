use core::ops::Deref;

struct NullSync<T> {
    inner: T,
}

impl<T> NullSync<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Deref for NullSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
