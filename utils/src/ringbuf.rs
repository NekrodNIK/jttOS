use core::mem::MaybeUninit;

pub struct Ringbuf<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    read_ind: usize,
    write_ind: usize,
    count: usize,
}

impl<T, const N: usize> Ringbuf<T, N> {
    pub const fn new() -> Self {
        Self {
            array: [const { MaybeUninit::uninit() }; N],
            read_ind: 0,
            write_ind: 0,
            count: 0,
        }
    }

    pub const fn push(&mut self, value: T) {
        self.array[self.write_ind] = MaybeUninit::new(value);
        self.write_ind += 1;

        if self.write_ind == self.read_ind {
            self.read_ind += 1;
        }

        self.write_ind %= N;
        self.read_ind %= N;
        self.count += 1;
    }

    pub const fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let res = unsafe { Some(self.array[self.read_ind].assume_init_read()) };
        self.read_ind = (self.read_ind + 1) % N;
        self.count -= 1;
        res
    }

    pub const fn count(&self) -> usize {
        self.count
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
}
