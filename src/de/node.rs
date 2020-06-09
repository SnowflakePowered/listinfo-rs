use crate::elements::*;
use crate::iter::*;
use crate::Error;
use core::any::type_name;
use core::result::Result as CoreResult;
use hex;
use serde::de::{
    self, DeserializeSeed, Deserializer, IntoDeserializer,
    SeqAccess, Visitor,
};
use serde::forward_to_deserialize_any;
use serde::serde_if_integer128;
type Result<T> = CoreResult<T, crate::Error>;

pub struct NodeDeserializer<'a, T> {
    iter: NodeIter<'a, T>,
    item: &'a T,
}

impl<'a, T> NodeDeserializer<'a, T> {
    pub(crate) fn new(n: &'a Node<T>) -> Self {
        NodeDeserializer {
            iter: n.iter(),
            item: n.unique(),
        }
    }
}

impl<'de> SeqAccess<'de> for NodeDeserializer<'de, &'de str> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(&value) => seed.deserialize(value.into_deserializer()).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

macro_rules! deserialize_primitive {
    ($t:ty: $deserialize:ident => $visit:ident) => {
        fn $deserialize<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Error> {
            let value = self.item.parse::<$t>().map_err(|_| {
                Error::DeserializationTypeError(self.item.to_string(), type_name::<$t>())
            })?;
            visitor.$visit(value)
        }
    };
}

impl<'de> Deserializer<'de> for NodeDeserializer<'de, &'de str> {
    type Error = crate::Error;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(*self.item)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(String::from(*self.item))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.item {
            &"yes" => visitor.visit_bool(true),
            &"no" => visitor.visit_bool(false),
            _ => visitor.visit_bool(self.item.parse::<bool>().map_err(|_| {
                Error::DeserializationTypeError(self.item.to_string(), type_name::<bool>())
            })?),
        }
    }

    deserialize_primitive!(i8: deserialize_i8 => visit_i8);
    deserialize_primitive!(i16: deserialize_i16 => visit_i16);
    deserialize_primitive!(i32: deserialize_i32 => visit_i32);
    deserialize_primitive!(i64: deserialize_i64 => visit_i64);

    deserialize_primitive!(u8: deserialize_u8 => visit_u8);
    deserialize_primitive!(u16: deserialize_u16 => visit_u16);
    deserialize_primitive!(u32: deserialize_u32 => visit_u32);
    deserialize_primitive!(u64: deserialize_u64 => visit_u64);

    serde_if_integer128! {
        deserialize_primitive!(i128: deserialize_i128 => visit_i128);
        deserialize_primitive!(u128: deserialize_u128 => visit_u128);
    }

    deserialize_primitive!(f32: deserialize_f32 => visit_f32);
    deserialize_primitive!(f64: deserialize_f64 => visit_f64);
    deserialize_primitive!(char: deserialize_char => visit_char);

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let buf = hex::decode(self.item).map_err::<Error, _>(|_| {
            de::Error::invalid_value(de::Unexpected::Str(self.item), &visitor)
        })?;
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(de::Unexpected::Map, &visitor))
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
        self.deserialize_map(visitor)
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
