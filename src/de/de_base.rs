use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor, value,
};
use serde::forward_to_deserialize_any;
use serde::Deserialize;

use crate::elements::*;
use crate::iter::*;
use crate::parse::parse_document;
use core::result::Result as CoreResult;

use super::node::NodeDeserializer;
use super::sub_entry::SubEntryDeserializer;
pub struct Deserializer<'de> {
    input: DatDocument<'de>,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_document(input: DatDocument<'de>) -> Self {
        Deserializer { input }
    }
}

type Result<T> = CoreResult<T, crate::Error>;


impl<'de> de::Deserializer<'de> for &'de SubEntry<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let iter = self.iter();
        let mut deserializer = SubEntryDeserializer::new(iter);
        visitor.visit_map(&mut deserializer)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let iter = self.iter();
        let mut deserializer = SubEntryDeserializer::new(iter);

        visitor.visit_map(&mut deserializer)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct enum identifier ignored_any
    }
}

#[derive(Debug, Deserialize)]
struct Help(i32);
#[derive(Debug, Deserialize)]
struct TestStruct {
    // hello: Vec<u8>,
    // number: serde_bytes::ByteBuf,
    test: Option<String>
}

#[derive(Debug, Deserialize)]
struct Document {
    // clrmamepro: Header
    // game: Game,
    // clrmamepro: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Header {
    name: String,
    description: String,
    category: String,
    version: i32,
    author: Vec<String>,
    rom: Vec<Rom>
}

#[derive(Debug, Deserialize)]
struct Game {
    name: String,
}
#[derive(Debug, Deserialize)]
struct Rom {
    crc: String
}
use super::entry_fragment::EntryFragmentDeserializer;
use super::dat_document::DatDocumentDeserializer;
#[test]
fn test_deserialize() {
    const HEADER: &str = r#"clrmamepro (
        name "Test"
        description "Test Description"
        category TestCategory
        version 42069
        author "TestAuthor"
        author "TestAuthor"
    )"#;

    let header_str = String::from(HEADER);
    // let (_, header) = crate::parse::parse_fragment(&header_str).unwrap();
    let doc = crate::parse::parse_document(&header_str).unwrap();
    let t = Document::deserialize(DatDocumentDeserializer::new(doc.iter()));
    println!("{:?}", t);
}
