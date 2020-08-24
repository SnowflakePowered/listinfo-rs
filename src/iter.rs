//! Internal iterator types for ListInfo elements.

use crate::elements::Node;
use alloc::vec::Vec;
use indexmap::map::Iter;

/// Iterator that yields slices and string keys.
pub struct SliceIter<'a, T> {
    inner_iter: Iter<'a, &'a str, Vec<T>>,
}

impl <'a, T> SliceIter<'a, T> {
    #[doc(hidden)]
    pub(crate) fn new(inner_iter: Iter<'a, &'a str, Vec<T>>) -> Self {
        SliceIter {
            inner_iter
        }
    }
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = (&'a str, &'a [T]);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((k, v))
        } else {
            None
        }
    }
}

/// Iterator for ListInfo entries that yields keys and borrows
pub struct EntryIter<'a, K, V> {
    inner_iter: Iter<'a, K, V>,
}

impl <'a, K, V> EntryIter<'a, K, V> {
    #[doc(hidden)]
    pub(crate) fn new(inner_iter: Iter<'a, K, V>) -> Self {
        EntryIter {
            inner_iter
        }
    }
}

impl<'a, K, V> Iterator for EntryIter<'a, &'a K, V> 
where K: ?Sized {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((&k, v.into()))
        } else {
            None
        }
    }
}

/// Iterator for `Node` that abstracts over `Node::Unique` and `Node::Many`
/// to access Node values.
pub struct NodeIter<'a, T> {
    node: &'a Node<T>,
    dead: bool,
    multi_idx: usize,
}

impl <'a, T> NodeIter<'a, T> {
    #[doc(hidden)]
    pub(crate) fn new(node: &'a Node<T>) -> Self {
        NodeIter {
            node,
            dead: false,
            multi_idx: 0
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.dead {
            return None;
        }

        match self.node {
            Node::Unique(entry) => {
                self.dead = true;
                return Some(entry);
            }
            Node::Many(vec) => {
                let get = vec.get(self.multi_idx);
                self.multi_idx += 1;
                if get.is_none() {
                    self.dead = true;
                }
                get
            }
        }
    }
}
