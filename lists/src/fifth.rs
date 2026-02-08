//queue pops off the other end.
//FIFO
//Unsafe Rust

use std::{mem, ptr};

pub struct List<T> {
    head: Link<T>,
    // tail: Option<&'a mut Node<T>>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    //as_deref_mut is a helper method on Option that, when the option contains a type
    //implementing DerefMut, converts it into an option of mutable references to the inner target.

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        //pub fn push(&'a mut self, elem: T) {
        // old_tail is a local variable,so its lifetime is shorter than self.tail's
        // let new_tail = match self.tail.take() {
        //     Some(old_tail) => {
        //         old_tail.next = Some(new_tail);
        //         old_tail.next.as_deref_mut()
        //     }
        //     None => {
        //         self.head = Some(new_tail);
        //         self.head.as_deref_mut()
        //     }
        // };

        // //Now the compiler can see that the reference stored in new_tail lives as long as self
        // self.tail = new_tail;

        //unsafe rust
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            //unsafe tells compiler: I guarantee this raw pointer operation is safe
            //Required for dereferencing raw pointers and FFI calls
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::fifth::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
    }
}
