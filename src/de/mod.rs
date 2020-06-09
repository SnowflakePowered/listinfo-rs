mod dat_document;
mod de_base;
mod entry_data;
mod entry_fragment;
mod node;
mod sub_entry;

use node::NodeDeserializer;
use sub_entry::SubEntryDeserializer;

use serde::de::{Deserialize, DeserializeOwned, IntoDeserializer};

use crate::Error;
use crate::elements::*;
use crate::parse::parse_document;
pub use dat_document::DatDocumentDeserializer as Deserializer;
pub use entry_fragment::EntryFragmentDeserializer as FragmentDeserializer;

type Result<T> = core::result::Result<T, crate::Error>;

/// Deserialize from a parsed `DatDocument`.
pub fn from_document<'de, T: Deserialize<'de>>(doc: &'de DatDocument<'de>) -> Result<T> {
    T::deserialize(Deserializer::new(doc.iter()))
}

/// Deserialize a ListInfo fragment from a parsed `EntryFragment`.
pub fn from_fragment<'de, T: Deserialize<'de>>(entry: &'de EntryFragment<'de>) -> Result<T> {
    T::deserialize(FragmentDeserializer::new(entry.iter()))
}

/// Deserialize from the string contents of a ListInfo DAT.
pub fn from_str<'de, T: DeserializeOwned>(s: &str) -> Result<T> {
    let parsed = parse_document(s)?;
    from_document(&parsed)
}

impl <'de> IntoDeserializer<'de, Error> for &'de DatDocument<'de> {
    type Deserializer = Deserializer<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::new(self.iter())
    }
}

impl <'de> IntoDeserializer<'de, Error> for &'de EntryFragment<'de> {
    type Deserializer = FragmentDeserializer<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        FragmentDeserializer::new(self.iter())
    }
}

impl <'de> IntoDeserializer<'de, Error> for &'de Node<&'de str> {
    type Deserializer = NodeDeserializer<'de, &'de str>;
    fn into_deserializer(self) -> Self::Deserializer {
        NodeDeserializer::new(self)
    }
}

impl <'de> IntoDeserializer<'de, Error> for &'de Node<EntryData<'de>> {
    type Deserializer = NodeDeserializer<'de, EntryData<'de>>;
    fn into_deserializer(self) -> Self::Deserializer {
        NodeDeserializer::new(self)
    }
}

impl <'de> IntoDeserializer<'de, Error> for &'de EntryData<'de> {
    type Deserializer = &'de EntryData<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl <'de> IntoDeserializer<'de, Error> for &'de SubEntry<'de> {
    type Deserializer = SubEntryDeserializer<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        SubEntryDeserializer::new(self.iter())
    }
}
