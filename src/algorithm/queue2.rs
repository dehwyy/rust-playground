use std::{cmp::max, fmt::Display, mem::MaybeUninit};

const DEFAULT_CAPACITY: usize = 10;
const GROW_FACTOR: usize = 2;

pub struct Queue {
    size: usize,
    inner: Box<[MaybeUninit<u8>]>,
    inner_start: usize, // Start of the slice
    inner_end: usize,   // End of the slice capacity
}

impl Queue {
    pub fn new() -> Self {
        Self {
            size: 0,
            inner: Box::<[u8]>::new_uninit_slice(DEFAULT_CAPACITY),
            inner_start: 0,
            inner_end: DEFAULT_CAPACITY,
        }
    }

    unsafe fn resize(&mut self) {
        let new_capacity = max(self.inner_end - self.inner_start, DEFAULT_CAPACITY) * GROW_FACTOR;

        let mut slice = Box::<[u8]>::new_uninit_slice(new_capacity);
        for (i, value) in self.inner[self.inner_start..self.inner_end]
            .iter()
            .enumerate()
        {
            slice[i].as_mut_ptr().write(value.assume_init());
        }

        self.inner_start = 0;
        self.inner_end = new_capacity;
        self.inner = slice;
    }

    pub fn enqueue(&mut self, value: u8) {
        if self.size == self.inner_end - self.inner_start {
            unsafe { self.resize() };
        }

        unsafe {
            self.inner[self.inner_start + self.size]
                .as_mut_ptr()
                .write(value);
        }

        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<u8> {
        if self.size == 0 {
            return None;
        }

        self.inner_start += 1;
        self.size -= 1;

        Some(unsafe { self.inner[self.inner_start - 1].assume_init() })
    }

    pub fn peek(&self) -> Option<&u8> {
        if self.size == 0 {
            return None;
        }

        Some(unsafe { self.inner[self.inner_start].assume_init_ref() })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Size: {}, underlying_array_capacity: {}, inner: ",
            self.size,
            self.inner_end - self.inner_start
        ))?;

        for i in self.inner_start..self.inner_start + self.size {
            f.write_fmt(format_args!("{} ", unsafe { self.inner[i].assume_init() }))?;
        }

        Ok(())
    }
}
impl super::Algorithm for Queue {
    fn showcase() -> Result<(), Box<dyn std::error::Error>> {
        let mut q = Queue::new();

        for v in 1..=100 {
            q.enqueue(v);
        }
        println!("{q}\n");

        for _ in 0..120 {
            q.dequeue();
        }
        println!("{q}\n");

        for v in 0..=52 {
            q.enqueue(v);
            if v % 10 == 0 {
                q.dequeue();
            }
        }
        println!("{q}\n");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn test() {
        let mut q = Queue::new();

        for v in 1..=100 {
            q.enqueue(v);
        }

        assert_eq!(q.size(), 100);
        assert_eq!(q.peek().unwrap(), &1);

        for _ in 0..120 {
            q.dequeue();
        }

        assert!(q.peek().is_none());

        for v in 0..=52 {
            q.enqueue(v);
            if v % 10 == 0 {
                q.dequeue();
            }
        }

        assert_eq!(q.size(), 47);
    }
}
