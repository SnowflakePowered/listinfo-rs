use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::iter::*;

/// The contents of a ListInfo DAT file.
#[derive(Debug)]
pub struct DatDocument<'a> {
    pub(crate) document: BTreeMap<&'a str, Vec<EntryFragment<'a>>>,
}

impl<'a> DatDocument<'a> {
    /// Get DAT entries with the given key as an iterator
    pub fn entry(&'a self, key: &str) -> Option<impl Iterator<Item = &EntryFragment<'a>>> {
        self.document.get(key).map(|f| f.iter())
    }

    /// Gets an iterator over the fragments of
    pub fn iter(&'a self) -> SliceIter<'a, EntryFragment<'a>> {
        SliceIter::new(self.document.iter())
    }
}

/// The contents of a sub-entry (such as `rom` or `disk`) that is a child of a ListInfo entry.
#[derive(Debug, Eq, PartialEq)]
pub struct SubEntry<'a> {
    pub(crate) keys: BTreeMap<&'a str, Node<&'a str>>,
}

impl<'a> SubEntry<'a> {
    /// Retrieves the value of an item data value in the sub-entry.
    pub fn value(&'a self, key: &str) -> Option<&'a Node<&'a str>> {
        self.keys.get(key)
    }

    /// Gets the entry node with the given key if it exists.
    ///
    /// This is shorthand for `subentry.value("key").map(|f| f.unique().as_ref())`
    pub fn value_unique(&'a self, key: &str) -> Option<&'a str> {
        self.keys.get(key).map(|f| f.unique().as_ref())
    }

    /// Gets the values with the given key if it exists.
    ///
    /// This is shorthand for `fragment.value("key").map(|f| f.iter().map(|&s| s))`
    pub fn value_iter(&'a self, key: &str) -> Option<impl Iterator<Item = &'a str>> {
        self.keys.get(key).map(|f| f.iter().map(|&s| s))
    }

    /// Gets an iterator over the values of this fragment.
    pub fn iter(&'a self) -> EntryIter<'a, Node<&'a str>> {
        EntryIter::new(self.keys.iter())
    }
}

/// Represents an item data value of an entry.
#[derive(Debug, Eq, PartialEq)]
pub enum EntryData<'a> {
    /// A scalar string entry
    Scalar(&'a str),
    /// A sub-entry (such as `rom`, for example)
    SubEntry(SubEntry<'a>),
}

/// Represents nodes with the given key in an ListInfo entry.
///
/// The split between `Unique` and `Many` is mostly for performance reasons
/// to avoid unnecessary allocations. `Node::Unique` appeared exactly once in 
/// the parsed DAT file, while `Node::Unique` appeared more than once.
///
/// Instead of accessing the enum members directly, the `Node::iter` and `Node::unique`
/// methods abstract over the difference between `Unique` and `Many` for convenience.
#[derive(Debug, Eq, PartialEq)]
pub enum Node<T> {
    /// A uniquely keyed node (only one of such key exists in the entry)
    Unique(T),
    /// Multiple nodes with the same key.
    Many(Vec<T>),
}

impl<'a, T> Node<T> {
    /// Gets the values with the given key.
    ///
    /// If the provided key is a unique value, returns an iterator that yields
    /// that single value.
    pub fn iter(&'a self) -> NodeIter<'a, T> {
        NodeIter::new(self)
    }

    /// Gets a single value with the given key.
    ///
    /// If the provided key is not unique, retrieves the first
    /// value of the many-set with the given key.
    pub fn unique(&'a self) -> &T {
        match self {
            Node::Unique(entry) => entry,
            // Node::Many must have vec of arity 2 or more
            // Any other situation is a bug, and should panic.
            Node::Many(entries) => entries.first().unwrap(),
        }
    }
}

/// Represents a single ListInfo entry fragment.
#[derive(Debug)]
pub struct EntryFragment<'a> {
    keys: BTreeMap<&'a str, Node<EntryData<'a>>>,
}

impl<'a> EntryFragment<'a> {
    #[doc(hidden)]
    pub(crate) fn new(keys: BTreeMap<&'a str, Node<EntryData<'a>>>) -> Self {
        EntryFragment { keys }
    }

    /// Gets the entry node with the given key if it exists.
    pub fn entry(&'a self, key: &str) -> Option<&'a Node<EntryData<'a>>> {
        self.keys.get(key)
    }

    /// Gets the entry node with the given key if it exists.
    ///
    /// This is shorthand for `fragment.entry("key").map(|f| f.unique())`
    pub fn entry_unique(&'a self, key: &str) -> Option<&'a EntryData> {
        self.keys.get(key).map(|f| f.unique())
    }

    /// Gets the values with the given key if it exists.
    ///
    /// This is shorthand for `fragment.entry("key").map(|f| f.iter())`
    pub fn entry_iter(&'a self, key: &str) -> Option<impl Iterator<Item = &'a EntryData>> {
        self.keys.get(key).map(|f| f.iter())
    }

    /// Gets an iterator over the entries of this fragment.
    pub fn iter(&'a self) -> EntryIter<'a, Node<EntryData<'a>>> {
        EntryIter::new(self.keys.iter())
    }
}
