use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::forward_to_deserialize_any;
use serde::Deserialize;

use crate::elements::*;
use crate::parse::parse_document;
use core::result::Result as CoreResult;

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

// pub fn from_document<'a, T>(s:  DatDocument<'a>) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     let mut deserializer = Deserializer::from_document(s);
//     let t = T::deserialize(&mut deserializer)?;
//     Ok(t)
// }

type Result<T> = CoreResult<T, crate::Error>;

struct StrDeserializer<'de> {
    str: &'de str,
}

impl<'de> de::Deserializer<'de> for StrDeserializer<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.str)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any enum
    }
}

impl<'de> de::Deserializer<'de> for &'de Node<&'de str> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Node::Many(entries) => unimplemented!(),
            &Node::Unique(entry) => visitor.visit_borrowed_str(entry),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            Node::Many(entries) => unimplemented!(),
            &Node::Unique(entry) => visitor.visit_borrowed_str(entry),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            Node::Many(entries) => unimplemented!(),
            &Node::Unique(entry) => visitor.visit_string(String::from(entry)),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any enum
    }
}

struct SubEntryDeserializer<'de> {
    iter: SubEntryIter<'de>,
    value: Option<&'de Node<&'de str>>,
}

impl<'de> SubEntryDeserializer<'de> {
    fn new(iter: SubEntryIter<'de>) -> Self {
        SubEntryDeserializer { iter, value: None }
    }
}

impl<'de> MapAccess<'de> for SubEntryDeserializer<'de> {
    type Error = crate::Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = StrDeserializer { str: key };
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(crate::Error::SerdeError("value is missing".to_string())),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

impl<'de> de::Deserializer<'de> for SubEntryDeserializer<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

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
struct TestStruct {
    hello: String,
}

#[test]
fn test_deserialize() {
    let mut map = alloc::collections::BTreeMap::new();
    map.insert("hello", Node::Unique("world"));
    let entry = SubEntry { keys: map };
    let t = TestStruct::deserialize(&entry);
    println!("{:?}", t);
}
