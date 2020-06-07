use std::collections::BTreeMap;
use std::iter;

/// The contents of a ListInfo DAT file.
#[derive(Debug)]
pub struct DatDocument<'a> {
    pub(crate) document: BTreeMap<&'a str, Vec<EntryFragment<'a>>>,
}

impl<'a> DatDocument<'a> {
    /// Get DAT entries with the given key as an iterator
    pub fn get_entries(&self, key: &'a str) -> Option<impl Iterator<Item = &EntryFragment<'a>>> {
        self.document.get(key).map(|f| f.iter())
    }
}

/// The contents of a sub-entry (such as `rom` or `disk`) that is a child of a ListInfo entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubEntry<'a> {
    pub(crate) keys: BTreeMap<&'a str, &'a str>,
}

impl <'a> SubEntry<'a> {
    /// Retrieves the value of an item data value in the sub-entry
    pub fn get(&'a self, key: &str) -> Option<&'a str> {
        self.keys.get(key).map(|&f| f)
    }
}

/// Represents an item data value of an entry
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntryData<'a> {
    /// A scalar string entry 
    Scalar(&'a str),
    /// A sub-entry (such as `rom`, for example)
    SubEntry(SubEntry<'a>),
}
/// Represents one node in an ListInfo entry
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

/// Represents a single ListInfo entry fragment
#[derive(Debug)]
pub struct EntryFragment<'a> {
    keys: BTreeMap<&'a str, EntryNode<'a>>,
}

impl<'a> EntryFragment<'a> {
    #[doc(hidden)]
    pub(crate) fn new(keys: BTreeMap<&'a str, EntryNode<'a>>) -> Self {
        EntryFragment { keys }
    }

    /// Gets the entry node with the given key if it exists
    pub fn get(&'a self, key: &str) -> Option<&'a EntryNode<'a>> {
        self.keys.get(key)
    }

    /// Gets a single value with the given key if it exists.
    ///
    /// If the provided key is not unique, retrieves the first
    /// value of the many-set with the given key.
    pub fn get_unique(&'a self, key: &str) -> Option<&EntryData> {
        if let Some(EntryNode::Unique(entry)) = self.keys.get(key) {
            Some(entry)
        } else if let Some(EntryNode::Many(entries)) = self.keys.get(key) {
            entries.first()
        } else {
            None
        }
    }

    /// Gets the values with the given key if it exists.
    /// 
    /// If the provided key is a unique value, returns an iterator that yields
    /// that single value.
    /// 
    /// If performance from the boxed iterator is a consideration, you may choose instead
    pub fn get_iter(&'a self, key: &str) -> Option<Box<dyn Iterator<Item = &EntryData> + 'a>> {
        if let Some(node) = self.keys.get(key) {
            let iter: Box<dyn Iterator<Item = &EntryData>> = match node {
                EntryNode::Unique(entry) => Box::new(iter::once(entry)),
                EntryNode::Many(entry) => Box::new(entry.iter()),
            };
            return Some(iter);
        }
        return None;
    }
}
