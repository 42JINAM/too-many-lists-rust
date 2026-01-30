//Box is a pointer type for heap allocation.
//it provides the simplest form of heap allocation, ownership for the allocation
//and drop their contents.

use std::mem;

#[derive(Debug)]
pub enum BadList<T> {
    Elem(T, Box<BadList<T>>),
    Nil, // empty container != null,
         // null means dangling or invalid pointer (crash).
         // nil is the VALID value that presents the end of the list.
         // [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
         // junk -> in the heap, that doesn't need to be heap-allocated at all.
}
//To avoid the extra junk, uniformly allocate and get that sweet null-pointer..

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

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
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), // head: empty, next: head -> stack
        });

        self.head = Link::More(new_node);
    }
    // option : an enum that represents a value that may exist.
    pub fn pop(&mut self) -> Option<i32> {
        //Taking ownership whit mem::replace
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

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
