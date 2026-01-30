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
}
