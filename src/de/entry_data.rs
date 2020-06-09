use crate::{elements::*, Error, de::*};
use core::result::Result as CoreResult;
use serde::de::{self, Deserializer, Visitor};
type Result<T> = CoreResult<T, Error>;
use serde::serde_if_integer128;

macro_rules! deserialize_primitive {
    ($t:ty: $deserialize:ident => $visit:ident) => {
        fn $deserialize<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Error> {
            match self {
                EntryData::Scalar(item) => {
                    let value = item.parse::<$t>().map_err::<Error, _>(|_| {
                        de::Error::invalid_type(de::Unexpected::Str(item), &visitor)
                    })?;
                    visitor.$visit(value)
                }
                _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
            }
        }
    };
}

impl<'de> Deserializer<'de> for &'de EntryData<'de> {
    type Error = crate::Error;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            EntryData::Scalar(item) => visitor.visit_borrowed_str(*item),
            _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            EntryData::Scalar(item) => visitor.visit_string(String::from(*item)),
            _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // todo: SeqAccess for SubEntry
        match self {
            EntryData::Scalar(item) => {
                Err(de::Error::invalid_type(de::Unexpected::Str(item), &visitor))
            }
            _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            EntryData::Scalar(item) => match item {
                &"yes" => visitor.visit_bool(true),
                &"no" => visitor.visit_bool(false),
                _ => visitor.visit_bool(item.parse::<bool>().map_err::<Error, _>(|_| {
                    de::Error::invalid_type(de::Unexpected::Str(item), &"bool")
                })?),
            },
            _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
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
        match self {
            EntryData::Scalar(item) => {
                let buf = hex::decode(item).map_err::<Error, _>(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(item), &visitor)
                })?;
                visitor.visit_byte_buf(buf)
            }
            _ => Err(de::Error::invalid_type(de::Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
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
        match self {
            EntryData::Scalar(input) => Err(de::Error::invalid_type(
                de::Unexpected::Str(input),
                &visitor,
            )),
            EntryData::SubEntry(entry) => {
                visitor.visit_map(entry.into_deserializer())
            }
        }
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
