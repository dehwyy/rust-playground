use std::{collections::VecDeque, fmt::Display};

pub struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::<T>::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.inner.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Display for Queue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Head: ")?;
        for (i, value) in self.inner.iter().enumerate() {
            f.write_fmt(format_args!("{value}"))?;

            if i != self.size() - 1 {
                f.write_str(" <- ")?;
            }
        }

        Ok(())
    }
}

impl super::Algorithm for Queue<u32> {
    fn showcase() -> Result<(), Box<dyn super::Error>> {
        let mut queue = Queue::<u32>::new();

        queue.enqueue(32);
        queue.enqueue(52);
        queue.enqueue(152);
        queue.enqueue(1152);
        println!("{queue}");
        println!("Dequeued element: {}", queue.dequeue().unwrap());
        println!("1st element rn: {}", queue.peek().unwrap());
        println!("1st element rn 1 more time: {}", queue.peek().unwrap());
        println!("Queue: {queue}; size = {}", queue.size());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Algorithm;
    use super::*;

    #[test]
    fn queue() {
        let mut queue = Queue::<u32>::new();

        assert!(queue.dequeue().is_none());
        assert!(queue.peek().is_none());
        assert_eq!(queue.size(), 0);

        queue.enqueue(52);
        assert_eq!(queue.peek().unwrap(), &52);
        assert_eq!(queue.size(), 1);
        assert!(queue.dequeue().is_some());
        assert!(queue.dequeue().is_none());

        queue.enqueue(102);
        queue.enqueue(533);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.peek().unwrap(), &102);
    }

    #[test]
    fn showcase() {
        Queue::showcase().unwrap();
    }
}
