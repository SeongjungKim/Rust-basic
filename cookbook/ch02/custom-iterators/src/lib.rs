//!
//! A simple singly-linked list for the Rust-Cookbook by Packt Publishing. 
//! 
//! Recipes covered in this module:
//!  - Documenting your code
//!  - Testing your documentation
//!  - Writing tests and benchmarks
//! 

//#![feature(test)]
#![doc(html_logo_url = "https://blog.x5ff.xyz/img/main/logo.png",
       test(no_crate_inject, attr(allow(unused_variables), deny(warnings))))]

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> where T: Sized + Clone {
    value: T,
    next: Link<T>,
}
impl<T> Node<T> where T: Sized + Clone {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

/// 
/// A singly-linked list, with nodes allocated on the heap using `Rc`s and `RefCell`s. Here's an image illustrating a linked list:
/// 
/// 
/// ![](https://upload.wikimedia.org/wikipedia/commons/6/6d/Singly-linked-list.svg)
/// 
/// *Found on https://en.wikipedia.org/wiki/Linked_list*
/// 
/// # Usage
/// 
/// ```ignore
/// let list = List::new_empty();
/// ```
/// 
#[derive(Clone)]
pub struct List<T> where T: Sized + Clone {
    head: Link<T>,
    tail: Link<T>,

    pub length: usize,
}
impl<T> List<T> where T: Sized + Clone {
    ///
    /// Creates a new empty list.
    /// 
    ///  
    /// # Example
    /// 
    /// ```
    /// # use custom_iterators::List;
    /// let list: List<i32> = List::new_empty();
    /// ```
    /// 
    pub fn new_empty() -> List<T> {
        List { head: None, tail: None, length: 0 }
    }
    ///
    /// Appends a node to the list at the end.
    /// 
    ///  
    /// # Panics
    /// 
    /// This never panics (probably).
    /// 
    /// # Safety
    /// 
    /// No unsafe code was used.
    /// 
    /// # Example
    /// 
    /// ```
    /// use custom_iterators::List;
    /// 
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// ```
    /// 
    pub fn append(&mut self, value: T) {
        // generate the new node
        // NODE_VALUE:[NEW NODE]
        let new = Node::new(value);

        // tail of list link check
        match self.tail.take() {
            // if tail of list link isn't empty, new node move to next node link
            // LIST_TAIL_LINK: [OLD NODE]
            // LIST_HEAD_LINK: [OLD NODE]
            // NEXT_LINK: [OLD_NODE or EMPTY -> NEW NODE]
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            // if tail of list link is empty, new node move to head list link
            // LIST_TAIL_LINK: [EMPTY]
            // LIST_HEAD_LINK: [EMPTY -> NEW NODE]
            None => self.head = Some(new.clone()),
        };

        // increase list length
        self.length += 1;

        // new node move to tail list link
        // LIST_TAIL_LINK: [NEW NODE]
        self.tail = Some(new);
    }
    ///
    /// Removes the list's head and returns the result.
    ///
    ///  
    /// # Panics
    ///
    /// Whenever when a node unexpectedly is `None`
    ///
    /// # Example
    ///
    /// ```
    /// # use custom_iterators::List;
    ///
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// assert_eq!(list.pop_front(), Some(10));
    /// ```
    ///
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            // if next node link isn't empty, next node link move to head list link
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            // if newxt node link is empty, tail list link move to head list link
            } else {
                self.tail.take();
            }

            // decrease list length
            self.length -= 1;

            // return value of head list link
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}
///
/// An iterator for the linked list. Consumes the list.
///
pub struct ConsumingListIterator<T> where T: Clone + Sized {
    list: List<T>,
}
impl<T> ConsumingListIterator<T> where T: Clone + Sized {
    ///
    /// Create a new iterator for this list
    ///
    fn new(list: List<T>) -> ConsumingListIterator<T> {
        ConsumingListIterator { list: list }
    }
}
impl<T> Iterator for ConsumingListIterator<T> where T: Clone + Sized {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}
impl<T> IntoIterator for List<T> where T: Clone + Sized {
    type Item = T;
    type IntoIter = ConsumingListIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingListIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_list(n: usize, value: Option<usize>) -> List<usize> {
        let mut list = List::new_empty();
        for i in 1..=n {
            if let Some(v) = value {
                list.append(v);
            } else {
                list.append(i);
            }
        }

        list
    }

    #[test]
    fn test_list_iterator() {
        let list = new_list(4, None);
        assert_eq!(list.length, 4);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);

        let list = new_list(4, Some(1));
        assert_eq!(list.length, 4);
        
        for item in list {
            assert_eq!(item, 1);
        }

        let list = new_list(4, Some(1));
        assert_eq!(list.length, 4);
        assert_eq!(list.into_iter().fold(0, |s, e| s + e), 4);
    }
}