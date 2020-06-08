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
    pub(crate) keys: BTreeMap<&'a str, EntryNode<&'a str>>,
}

impl<'a> SubEntry<'a> {
    /// Retrieves the value of an item data value in the sub-entry.
    pub fn value(&'a self, key: &str) -> Option<&'a EntryNode<&'a str>> {
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

    pub fn iter(&'a self) -> SubEntryIter<'a> {
        SubEntryIter {
            inner_iter: self.keys.iter(),
        }
    }
}

pub struct SubEntryIter<'a> {
    inner_iter: alloc::collections::btree_map::Iter<'a, &'a str, EntryNode<&'a str>>,
}

impl<'a> Iterator for SubEntryIter<'a> {
    type Item = (&'a str, &'a EntryNode<&'a str>);
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

/// Represents one node in an ListInfo entry.
///
/// Note: The split between `Unique` and `Many` is mostly for
/// performance reasons to avoid unnescessary allocations.
///
/// Instead of matching the `EntryNode`, use`EntryFragment::get_unique()`
/// and `EntryFragment::get_iter()` to access `EntryData` per expectations.
#[derive(Debug, Eq, PartialEq)]
pub enum EntryNode<T> {
    /// A uniquely keyed node (only one of such key exists in the entry)
    Unique(T),
    /// Multiple nodes with the same key.
    Many(Vec<T>),
}

impl<'a, T> EntryNode<T> {
    /// Gets the values with the given key.
    ///
    /// If the provided key is a unique value, returns an iterator that yields
    /// that single value.
    pub fn iter(&'a self) -> EntryNodeIter<'a, T> {
        return EntryNodeIter {
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
            EntryNode::Unique(entry) => entry,
            // EntryNode::Many must have vec of arity 2 or more
            // Any other situation is a bug, and should panic.
            EntryNode::Many(entries) => entries.first().unwrap(),
        }
    }
}

/// Represents a single ListInfo entry fragment.
#[derive(Debug)]
pub struct EntryFragment<'a> {
    keys: BTreeMap<&'a str, EntryNode<EntryData<'a>>>,
}

impl<'a> EntryFragment<'a> {
    #[doc(hidden)]
    pub(crate) fn new(keys: BTreeMap<&'a str, EntryNode<EntryData<'a>>>) -> Self {
        EntryFragment { keys }
    }

    /// Gets the entry node with the given key if it exists.
    pub fn entry(&'a self, key: &str) -> Option<&'a EntryNode<EntryData<'a>>> {
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

    /// Gets an iterator over the nodes of this fragment.
    pub fn iter(&'a self) -> EntryFragmentIter<'a> {
        EntryFragmentIter {
            inner_iter: self.keys.iter(),
        }
    }
}

/// Iterator for `EntryFragment`
pub struct EntryFragmentIter<'a> {
    inner_iter: alloc::collections::btree_map::Iter<'a, &'a str, EntryNode<EntryData<'a>>>,
}

impl<'a> Iterator for EntryFragmentIter<'a> {
    type Item = (&'a str, &'a EntryNode<EntryData<'a>>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&k, v)) = self.inner_iter.next() {
            Some((k, v))
        } else {
            None
        }
    }
}

/// Iterator for `EntryNode`
pub struct EntryNodeIter<'a, T> {
    node: &'a EntryNode<T>,
    dead: bool,
    multi_idx: usize,
}

impl<'a, T> Iterator for EntryNodeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.dead {
            return None;
        }

        match self.node {
            EntryNode::Unique(entry) => {
                self.dead = true;
                return Some(entry);
            }
            EntryNode::Many(vec) => {
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
