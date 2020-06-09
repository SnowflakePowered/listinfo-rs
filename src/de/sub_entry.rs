use crate::elements::*;
use crate::iter::*;
use crate::Error;
use core::result::Result as CoreResult;
use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

type Result<T> = CoreResult<T, Error>;

pub struct SubEntryDeserializer<'de> {
    iter: EntryIter<'de, Node<&'de str>>,
    value: Option<&'de Node<&'de str>>,
}

impl<'de> SubEntryDeserializer<'de> {
    pub(crate) fn new(iter: EntryIter<'de, Node<&'de str>>) -> Self {
        SubEntryDeserializer { iter, value: None }
    }
}

impl<'de> MapAccess<'de> for SubEntryDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = key.into_deserializer();
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
            Some(value) => seed.deserialize(value.into_deserializer()),
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

impl<'de> Deserializer<'de> for SubEntryDeserializer<'de> {
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
