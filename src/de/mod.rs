//! Serde deserialization support.
//!
//! listinfo-rs supports a higher-level deserialization API with serde.
//! This must first be enabled in Cargo.toml
//!
//! ```toml
//! listinfo = { version = "0.1", features = ["deserialize"] }
//! ```
//! 
//! ## Usage
//! listinfo-rs can be use as any other Serde deserializer
//! ```rust
//! use serde::Deserialize;
//!
//! use listinfo::de::from_str;
//!
//! #[derive(Debug, Deserialize)]
//! struct Header {
//!     name: String,
//!     description: String,
//!     version: String,
//!     comment: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Game {
//!     name: String,
//!     releaseyear: u32,
//!     developer: String,
//!     rom: Vec<Rom>,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Rom {
//!     name: String,
//!     size: u64,
//!     // Supports serialize hex strings to byte arrays
//!     #[serde(with = "serde_bytes")]
//!     crc: Vec<u8>,
//!     #[serde(with = "serde_bytes")]
//!     md5: Vec<u8>,
//!     #[serde(with = "serde_bytes")]
//!     sha1: Vec<u8>,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct CaveStory {
//!     clrmamepro: Header,
//!     game: Vec<Game>,
//! }
//! 
//! fn deserialize_cave_story() {
//!     const CAVE_STORY: &str = r#"clrmamepro (
//!                 name "Cave Story"
//!                 description "Cave Story"
//!                 version 20161204
//!                 comment "libretro | www.libretro.com"
//!             )
//!             game (
//!                 name "Cave Story (En)"
//!                 description "Cave Story (En)"
//!                 developer "Studio Pixel"
//!                 releaseyear "2004"
//!                 rom ( 
//!                     name "Doukutsu.exe"
//!                     size 1478656 
//!                     crc c5a2a3f6 
//!                     md5 38695d3d69d7a0ada8178072dad4c58b 
//!                     sha1 bb2d0441e073da9c584f23c2ad8c7ab8aac293bf
//!                 )
//!             )
//!         "#;
//!
//!     let cave_story = from_str::<CaveStory>(CAVE_STORY).unwrap();
//!     assert_eq!(cave_story.clrmamepro.name, "Cave Story");
//!     assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().name, "Doukutsu.exe");
//!     assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().size, 1478656);
//!     assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().crc, &[0xc5, 0xa2, 0xa3, 0xf6]);
//! }
//! ```
mod dat_document;
mod entry_data;
mod entry_fragment;
mod node;
mod sub_entry;

#[cfg(feature="test_deserialize")]
mod tests;

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
