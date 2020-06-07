use std::collections::BTreeMap;

pub struct DatDocument<'a> {
    pub header: Header<'a>,
    pub games: Vec<GameEntry<'a>>,
    pub resources: Vec<GameEntry<'a>>,
}

#[derive(Debug)]
pub struct Header<'a> {
    keys: BTreeMap<&'a str, &'a str>,
}

impl<'a> Header<'a> {
    pub(crate) fn new(keys: BTreeMap<&'a str, &'a str>) -> Self {
        Header { keys }
    }

    pub fn get(&'a self, key: &str) -> Option<&'a str> {
        self.keys.get(key).map(|&s| s)
    }
}

#[derive(Debug)]
pub struct RomEntry<'a> {
    pub(crate) name: Option<&'a str>,
    pub(crate) merge: Option<&'a str>,
    pub(crate) size: Option<u64>,
    pub(crate) crc: Option<&'a str>,
    pub(crate) md5: Option<&'a str>,
    pub(crate) sha1: Option<&'a str>,
}

impl<'a> RomEntry<'a> {
    pub fn name(&self) -> Option<&'a str> {
        self.name
    }
    pub fn merge(&self) -> Option<&'a str> {
        self.merge
    }
    pub fn crc(&self) -> Option<&'a str> {
        self.crc
    }
    pub fn md5(&self) -> Option<&'a str> {
        self.md5
    }
    pub fn sha1(&self) -> Option<&'a str> {
        self.sha1
    }
}
#[derive(Debug)]
pub struct GameEntry<'a> {
    keys: BTreeMap<&'a str, &'a str>,
    roms: Vec<RomEntry<'a>>,
    disks: Vec<RomEntry<'a>>,
    samples: Vec<&'a str>,
}

impl<'a> GameEntry<'a> {
    pub(crate) fn new(
        keys: BTreeMap<&'a str, &'a str>,
        roms: Vec<RomEntry<'a>>,
        disks: Vec<RomEntry<'a>>,
        samples: Vec<&'a str>,
    ) -> Self {
        GameEntry {
            keys,
            roms,
            disks,
            samples,
        }
    }

    pub fn get(&'a self, key: &str) -> Option<&'a str> {
        self.keys.get(key).map(|&s| s)
    }

    pub fn roms(&'a self) -> &'a [RomEntry<'a>] {
        &self.roms
    }

    pub fn disks(&'a self) -> &'a [RomEntry<'a>] {
        &self.disks
    }

    pub fn samples(&'a self) -> &'a [&'a str] {
        &self.samples
    }
}
