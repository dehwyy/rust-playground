use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
pub enum LinkedListError {
    OutOfBounds,
}

impl super::Error for LinkedListError {}
impl Display for LinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;
#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    next: Option<NodeRef<T>>,
    previous: Option<NodeRef<T>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            previous: None,
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// Circular double linked list
pub struct LinkedList<T> {
    head: Option<NodeRef<T>>,
    size: usize,
}

impl<T: Clone + Display> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn get(&self, at: usize) -> Option<NodeRef<T>> {
        if (self.size as i32) - (at as i32) <= 0 {
            return None;
        }

        let mut node = self.head.clone().unwrap();

        for _ in 0..at {
            match node.clone().borrow().next.clone() {
                None => break,
                Some(next_node) => node = next_node,
            }
        }

        Some(node)
    }

    pub fn insert(&mut self, value: T, to: usize) -> Result<(), LinkedListError> {
        if self.size < to {
            return Err(LinkedListError::OutOfBounds);
        }

        let new_node = Rc::new(RefCell::new(Node::new(value)));

        // Can't find el at position -> insert at the end
        let Some(old_node) = self.get(to) else {
            match self.size == 0 {
                // LinkedList is empty yet -> initialize `.head`.
                true => {
                    self.size = 1;
                    self.head = Some(new_node);
                }
                // Non-empty LL:
                false => {
                    // Get last element.
                    let last_node = self.get(to - 1).expect("Previous node should exist");

                    self.size += 1;
                    new_node.borrow_mut().previous = Some(last_node.clone());
                    last_node.borrow_mut().next = Some(new_node);
                }
            };

            return Ok(());
        };
        self.size += 1;
        // We've confirm, that we aren't working with `tail`: `Any`(∀) `new_node` ∈ [`head`, `tail`)
        // So `new_node` guaranteed not to be last.

        // `new_node`.next = `old_node`
        new_node.borrow_mut().next = Some(old_node.clone());

        // Trying to get `previous` node.
        // If `Some(node)` -> `new_node` should be in closed interval of head` and `tail`: ∀ `new_node`∈ (`head`, `tail`).
        // Else `None` -> `new_node` is `head`.
        match old_node.borrow().previous.clone() {
            Some(previous_node) => {
                // `new_node`.previous = `previous_node`
                // `previous_node`.next =`new_node`

                new_node.borrow_mut().previous = Some(previous_node.clone());
                previous_node.borrow_mut().next = Some(new_node.clone())
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        // `old`.previous =`new_node`
        old_node.borrow_mut().previous = Some(new_node.clone());

        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), LinkedListError> {
        self.insert(value, self.size)
    }
}

impl<T> Display for LinkedList<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_node = self.head.clone();

        while let Some(node) = current_node {
            f.write_fmt(format_args!("{} ", (*node).clone().into_inner().value))?;
            current_node = node.borrow().next.clone();
        }

        Ok(())
    }
}

impl super::Algorithm for LinkedList<u32> {
    fn showcase() -> Result<(), Box<dyn super::Error>> {
        let mut linked_list = LinkedList::<u32>::new();

        linked_list.push(1)?;
        linked_list.push(2)?;
        linked_list.push(3)?;

        linked_list.insert(727, 1)?;
        linked_list.insert(1000, 2)?;

        println!(
            "LinkedList at 2: {}",
            (*linked_list.get(2).unwrap()).clone().into_inner().value
        );
        println!("LinkedList size: {}", linked_list.size);
        println!("LinkedList: {linked_list}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Algorithm;
    use super::*;

    #[test]
    fn linked_list() {
        let mut linked_list = LinkedList::<u32>::new();
        assert!(linked_list.push(52).is_ok());
        assert!(linked_list.push(30).is_ok());

        assert!(linked_list.insert(120, 1).is_ok());
        assert!(linked_list.insert(144, 2).is_ok());
        assert!(linked_list.insert(1, 0).is_ok());

        assert!(linked_list.get(4).is_some());
        assert!(linked_list.get(0).is_some());

        assert_eq!(linked_list.get(0), linked_list.head);
        assert_eq!(linked_list.size, 5);
    }

    #[test]
    fn out_of_bounds() {
        let mut linked_list = LinkedList::<u32>::new();

        assert!(linked_list.insert(10, 1).is_err());
        assert!(linked_list.head.is_none());
        assert!(linked_list.get(0).is_none());
        assert_eq!(linked_list.size, 0);
    }

    #[test]
    fn showcase() {
        LinkedList::showcase().unwrap();
    }
}
