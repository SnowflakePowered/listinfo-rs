use crate::elements::*;
use crate::iter::*;

mod entry_data;
mod str;

pub use self::entry_data::*;
pub use self::str::*;

pub struct NodeDeserializer<'a, T> {
    iter: NodeIter<'a, T>,
    item: &'a T,
}

impl<'a, T> NodeDeserializer<'a, T> {
    pub(crate) fn new(n: &'a Node<T>) -> Self {
        NodeDeserializer {
            iter: n.iter(),
            item: n.unique(),
        }
    }
}
