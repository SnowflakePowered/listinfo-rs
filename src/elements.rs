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

/// The contents of a sub-entry (such as `rom` or `disk`) that is a child of a ListInfo entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubEntry<'a> {
    pub(crate) keys: BTreeMap<&'a str, &'a str>,
}

pub type SubEntryIter<'a> = alloc::collections::btree_map::Iter<'a, &'a str, &'a str>;

impl<'a> SubEntry<'a> {
    /// Retrieves the value of an item data value in the sub-entry.
    pub fn value(&'a self, key: &str) -> Option<&'a str> {
        self.keys.get(key).map(|&f| f)
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.keys.iter().map(|(&k, &v)| (k, v))
    }
}

/// Represents an item data value of an entry.
#[derive(Debug, Clone, Eq, PartialEq)]
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
pub enum EntryNode<'a> {
    /// A uniquely keyed node (only one of such key exists in the entry)
    Unique(EntryData<'a>),
    /// Multiple nodes with the same key.
    Many(Vec<EntryData<'a>>),
}

impl<'a> EntryNode<'a> {
    /// Gets the values with the given key.
    ///
    /// If the provided key is a unique value, returns an iterator that yields
    /// that single value.
    pub fn iter(&'a self) -> impl Iterator<Item = &EntryData> {
        return EntryIter {
            node: self,
            dead: false,
            multi_idx: 0,
        };
    }

    /// Gets a single value with the given key.
    ///
    /// If the provided key is not unique, retrieves the first
    /// value of the many-set with the given key.
    pub fn unique(&'a self) -> &EntryData {
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
    keys: BTreeMap<&'a str, EntryNode<'a>>,
}

impl<'a> EntryFragment<'a> {
    #[doc(hidden)]
    pub(crate) fn new(keys: BTreeMap<&'a str, EntryNode<'a>>) -> Self {
        EntryFragment { keys }
    }

    /// Gets the entry node with the given key if it exists.
    pub fn entry(&'a self, key: &str) -> Option<&'a EntryNode<'a>> {
        self.keys.get(key)
    }

    /// Gets the entry node with the given key if it exists.
    ///
    /// This is shorthand for `fragment.entry("key").map(|f| f.unique())`
    pub fn unique(&'a self, key: &str) -> Option<&'a EntryData> {
        self.keys.get(key).map(|f| f.unique())
    }

    /// Gets the values with the given key if it exists.
    ///
    /// This is shorthand for `fragment.entry("key").map(|f| f.iter())`
    pub fn iter(&'a self, key: &str) -> Option<impl Iterator<Item = &'a EntryData>> {
        self.keys.get(key).map(|f| f.iter())
    }
}

/// Iterator for `EntryNode`
struct EntryIter<'a> {
    node: &'a EntryNode<'a>,
    dead: bool,
    multi_idx: usize,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a EntryData<'a>;
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
