use std::collections::BTreeMap;
use std::iter;

pub struct DatDocument<'a> {
    pub games: Vec<InfoEntry<'a>>,
    pub resources: Vec<InfoEntry<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubEntryData<'a> {
    pub(crate) keys: BTreeMap<&'a str, &'a str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntryData<'a> {
    Value(&'a str),
    Node(SubEntryData<'a>),
}
#[derive(Debug, Eq, PartialEq)]
pub enum InfoNode<'a> {
    Unique(EntryData<'a>),
    Multiple(Vec<EntryData<'a>>),
}

#[derive(Debug)]
pub struct InfoEntry<'a> {
    keys: BTreeMap<&'a str, InfoNode<'a>>,
    // roms: Vec<RomEntry<'a>>,
    // disks: Vec<RomEntry<'a>>,
    // samples: Vec<&'a str>,
}

impl<'a> InfoEntry<'a> {
    pub(crate) fn new(
        keys: BTreeMap<&'a str, InfoNode<'a>>,
        // roms: Vec<RomEntry<'a>>,
        // disks: Vec<RomEntry<'a>>,
        // samples: Vec<&'a str>,
    ) -> Self {
        InfoEntry { keys }
    }

    pub fn get(&'a self, key: &str) -> Option<&'a InfoNode<'a>> {
        self.keys.get(key)
    }

    pub fn get_unique(&'a self, key: &str) -> Option<&EntryData> {
        if let Some(InfoNode::Unique(entry)) = self.keys.get(key) {
            Some(entry)
        } else {
            None
        }
    }

    pub fn get_iter(&'a self, key: &str) -> Option<Box<dyn Iterator<Item = &EntryData> + 'a>> {
        if let Some(node) = self.keys.get(key) {
            let iter: Box<dyn Iterator<Item = &EntryData>> = match node {
                InfoNode::Unique(entry) => Box::new(iter::once(entry)),
                InfoNode::Multiple(entry) => Box::new(entry.iter()),
            };
            return Some(iter);
        }
        return None;
    }
}
