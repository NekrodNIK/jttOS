use core::mem::MaybeUninit;

pub struct RingBuffer<T, const SIZE: usize> {
    arr: [MaybeUninit<T>; SIZE],
    start: usize,
    end: usize,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            arr: [const { MaybeUninit::uninit() }; SIZE],
            start: 0,
            end: 0,
        }
    }

    fn read(&self, index: usize) -> Option<T> {
        if self.available() {
            Some(unsafe { self.arr[index].assume_init_read() })
        } else {
            None
        }
    }

    pub fn push_back(&mut self, item: T) -> Option<T> {
        let result = if self.full() { self.pop_front() } else { None };

        self.arr[self.end].write(item);
        self.end = (self.end + 1) % SIZE;
        result
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.read(self.start).inspect(|_| {
            self.start = (self.start + 1) % SIZE;
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.read(self.end).inspect(|_| {
            self.end = (self.end + SIZE - 1) % SIZE;
        })
    }

    pub fn available(&self) -> bool {
        self.start != self.end
    }

    pub fn full(&self) -> bool {
        (self.start + 1) % SIZE == self.end
    }
}
