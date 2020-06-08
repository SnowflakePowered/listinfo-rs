use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use core::result::Result as CoreResult;
use crate::parse::parse_document;
use crate::elements::*;

pub struct Deserializer<'de> {
    input: DatDocument<'de>
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

struct SubEntryDeserializer<'de> {
    iter: SubEntryIter<'de>,
    value: Option<&'de str>
}


// impl<'de> de::Deserializer<'de> for SubEntry<'de> {
//     type Error = crate::Error;

//     fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
//     where
//         V: Visitor<'de>,
//     {
//         visit
//         match *self {
//             Value::Object(ref v) => visit_object_ref(v, visitor),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//     fn deserialize_struct<V>(
//         self,
//         _name: &'static str,
//         _fields: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
        
//     }

// }