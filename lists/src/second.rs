//Box is a pointer type for heap allocation.
//it provides the simplest form of heap allocation, ownership for the allocation
//and drop their contents.

use std::mem;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IntoIter<T>(List<T>);
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// but it's useless.
// 3 primary forms of ownership
// self - Value
// &mut self - mutable reference(but we can't do to move or to destroy)
// &self - shared reference (to observe self)
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None), // head: empty, next: head -> stack
        });

        self.head = Some(new_node);
    }
    // option : an enum that represents a value that may exist.
    pub fn pop(&mut self) -> Option<T> {
        //take : change self.head to None, return original value
        // using map
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    //peek
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    //iter
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // The  compiler infers the lifetime from &self
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// drop
// Box<Node> is not tail recursive, compiler can't turn this into a loop

impl<T> Drop for List<T> {
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
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn normal_removal() {
        let mut list: List<i32> = List::new();
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
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42;
        });
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
