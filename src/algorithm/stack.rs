use std::fmt::{Debug, Display};

pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::<T>::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.first()
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Display for Stack<T>
where
    Vec<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Size: {}, Stack: {:?}",
            self.size(),
            self.inner
        ))
    }
}

impl super::Algorithm for Stack<u8> {
    fn showcase() -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = Stack::<u8>::new();
        stack.push(30);
        stack.push(52);

        println!("{stack}");

        println!("Stack peek = {}", stack.peek().unwrap());

        stack.pop();
        println!("Popped once: {stack}");

        Ok(())
    }
}

// GPT generated tests WW;
#[cfg(test)]
mod tests {
    use crate::algorithm::Algorithm;

    use super::*;

    #[test]
    fn showcase() {
        Stack::showcase().unwrap()
    }

    #[test]
    fn test_new_stack() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.size(), 0);
        assert!(stack.peek().is_none());
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(&10));
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.size(), 1);

        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.size(), 0);

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.peek().is_none());

        stack.push(10);

        assert_eq!(stack.peek(), Some(&10));
    }

    #[test]
    fn test_size() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.size(), 0);

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.size(), 2);

        stack.pop();

        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_stack_with_strings() {
        let mut stack: Stack<String> = Stack::new();

        stack.push("Hello".to_string());
        stack.push("World".to_string());

        assert_eq!(stack.size(), 2);
        assert_eq!(stack.pop(), Some("World".to_string()));
        assert_eq!(stack.pop(), Some("Hello".to_string()));
        assert_eq!(stack.pop(), None);
    }
}
