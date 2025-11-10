use core::mem::MaybeUninit;

pub struct CircularBuffer<T, const SIZE: usize> {
    arr: [MaybeUninit<T>; SIZE],
    start: usize,
    end: usize,
}

impl<T, const SIZE: usize> CircularBuffer<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            arr: [const { MaybeUninit::uninit() }; SIZE],
            start: 0,
            end: 0,
        }
    }

    pub fn push_back(&mut self, item: T) -> Option<T> {
        let is_overwrite = (self.start + 1) % SIZE == self.end;

        let result = if is_overwrite {
            self.start += 1;
            Some(unsafe { self.arr[self.end].assume_init_read() })
        } else {
            None
        };

        self.arr[self.end].write(item);
        self.end += 1;
        self.end %= SIZE;
        result
    }

    fn read(&self, index: usize) -> Option<T> {
        if self.is_empty() {
            let item = unsafe { self.arr[index].assume_init_read() };
            Some(item)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let result = self.read(self.end);
        if matches!(result, Some(..)) {
            self.end -= 1;
        }
        result
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let result = self.read(self.start);
        if matches!(result, Some(..)) {
            self.start -= 1;
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn is_full(&self) -> bool {
        (self.start + 1) % SIZE == self.end
    }
}
