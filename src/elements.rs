use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// The contents of a ListInfo DAT file.
#[derive(Debug)]
pub struct DatDocument<'a> {
    pub(crate) document: BTreeMap<&'a str, Vec<EntryFragment<'a>>>,
}

impl<'a> DatDocument<'a> {
    /// Get DAT entries with the given key as an iterator
    pub fn entry(&self, key: &'a str) -> Option<impl Iterator<Item = &EntryFragment<'a>>> {
        self.document.get(key).map(|f| f.iter())
    }
}

/// Iterator for `DatDocument`
pub struct DatDocumentIter<'a> {
    inner_iter: alloc::collections::btree_map::Iter<'a, &'a str, Vec<EntryFragment<'a>>>,
}

impl<'a> Iterator for DatDocumentIter<'a> {
    type Item = (&'a str, &'a [EntryFragment<'a>]);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((k, v))
        } else {
            None
        }
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
    pub fn iter(&'a self) -> SubEntryIter<'a> {
        SubEntryIter {
            inner_iter: self.keys.iter(),
        }
    }
}

/// Iterator for `SubEntry`
pub struct SubEntryIter<'a> {
    inner_iter: alloc::collections::btree_map::Iter<'a, &'a str, Node<&'a str>>,
}

impl<'a> Iterator for SubEntryIter<'a> {
    type Item = (&'a str, &'a Node<&'a str>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((k, v))
        } else {
            None
        }
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
        return NodeIter {
            node: self,
            dead: false,
            multi_idx: 0,
        };
    }

    /// Gets a single value with the given key.
    ///
    /// If the provided key is not unique, retrieves the first
    /// value of the many-set with the given key.
    pub fn unique(&'a self) -> &T {
        match self {
            Node::Unique(entry) => entry,
            // EntryNode::Many must have vec of arity 2 or more
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
    pub fn iter(&'a self) -> EntryFragmentIter<'a> {
        EntryFragmentIter {
            inner_iter: self.keys.iter(),
        }
    }
}

/// Iterator for `EntryFragment`
pub struct EntryFragmentIter<'a> {
    inner_iter: alloc::collections::btree_map::Iter<'a, &'a str, Node<EntryData<'a>>>,
}

impl<'a> Iterator for EntryFragmentIter<'a> {
    type Item = (&'a str, &'a Node<EntryData<'a>>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((k, v))
        } else {
            None
        }
    }
}

/// Iterator for `EntryNode`
pub struct NodeIter<'a, T> {
    node: &'a Node<T>,
    dead: bool,
    multi_idx: usize,
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
