use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
pub enum LinkedListError {
    OutOfBounds,
    Unhandled,
}

pub type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Clone)]
pub struct Node<T> {
    pub value: T,
    next: NodeRef<T>,
    previous: NodeRef<T>,
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

// Circular double linked list
pub struct LinkedList<T> {
    head: NodeRef<T>,
    size: usize,
}

impl<T: Clone + Display> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn get(&self, at: usize) -> Result<NodeRef<T>, LinkedListError> {
        match (self.size as i32) - (at as i32) {
            sub if sub < 0 => return Err(LinkedListError::OutOfBounds), // Case (1)
            sub if sub == 0 => return Ok(None),                         // Case (2)
            _ => {}
        };

        // Shoud exist as `self.size` != 0:
        // We have at >= 0, self >= 0 (as `usize`):
        // { at > size  <-  Case 1
        // { at == size <- Case 2
        // { at < size  <- (size >  0, at >= 0) -> size > 0
        let mut node = self.head.clone().unwrap();

        for _ in 0..at {
            match node.clone().borrow().next.clone() {
                None => break,
                Some(next_node) => node = next_node,
            }
        }

        Ok(Some(node))
    }

    pub fn insert(&mut self, value: T, to: usize) -> Result<(), LinkedListError> {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        // Can't find el at position -> insert at the end
        let Some(old_node) = self.get(to)? else {
            match self.size == 0 {
                // LinkedList is empty yet -> initialize `.head`.
                true => {
                    self.size = 1;
                    self.head = Some(new_node);
                }
                // Non-empty LL:
                false => {
                    // Get last element.
                    let last_node = self
                        .get(to - 1)
                        .expect("Should not panic!")
                        .expect("Previous node should exist");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list() -> Result<(), LinkedListError> {
        let mut linked_list = LinkedList::<u32>::new();
        linked_list.push(52)?;
        linked_list.push(30)?;
        linked_list.push(77)?;
        linked_list.push(177)?;
        linked_list.push(727)?;

        linked_list.insert(120, 3)?;
        linked_list.insert(144, 3)?;
        linked_list.push(3030)?;

        println!("{}", linked_list);
        println!("{}", linked_list.size);

        Ok(())
    }
}
