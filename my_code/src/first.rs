use std::mem;

struct Node {
    data: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

use Link::{Empty, More};

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Empty }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: mem::replace(&mut self.head, Empty),
        });
        self.head = More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Empty) {
            Empty => None,
            More(boxed_node) => {
                self.head = boxed_node.next;
                Some(boxed_node.data)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_list = mem::replace(&mut self.head, Empty);
        while let More(mut boxed_node) = current_list {
            current_list = mem::replace(&mut boxed_node.next, Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        for i in 1..3 {
            list.push(i);
        }

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None)
    }
}
