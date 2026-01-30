//Box is a pointer type for heap allocation.
//it provides the simplest form of heap allocation, ownership for the allocation
//and drop their contents.

use std::mem;
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

// but it's useless.
// 3 primary forms of ownership
// self - Value
// &mut self - mutable reference(but we can't do to move or to destroy)
// &self - shared reference (to observe self)
impl List {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None), // head: empty, next: head -> stack
        });

        self.head = Some(new_node);
    }
    // option : an enum that represents a value that may exist.
    pub fn pop(&mut self) -> Option<i32> {
        //take : change self.head to None, return original value
        // using map
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

// drop
// Box<Node> is not tail recursive, compiler can't turn this into a loop

impl Drop for List {
    fn drop(&mut self) {
        //my optimization
        while let Some(boxed_node) = self.head.take() {
            self.head = boxed_node.next;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn empty_list() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn normal_removal() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
