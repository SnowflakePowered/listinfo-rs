// use serde::Deserialize;
// use serde::de::{
//     self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
//     VariantAccess, Visitor,
// };

// use core::result::Result as CoreResult;
// use crate::parse::parse_document;
// use crate::elements::*;

// pub struct Deserializer<'de> {
//     input: DatDocument<'de>
// }

// impl<'de> Deserializer<'de> {
//     // By convention, `Deserializer` constructors are named like `from_xyz`.
//     // That way basic use cases are satisfied by something like
//     // `serde_json::from_str(...)` while advanced use cases that require a
//     // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
//     pub fn from_document(input: DatDocument<'de>) -> Self {
//         Deserializer { input }
//     }
// }


// // pub fn from_document<'a, T>(s:  DatDocument<'a>) -> Result<T>
// // where
// //     T: Deserialize<'a>,
// // {
// //     let mut deserializer = Deserializer::from_document(s);
// //     let t = T::deserialize(&mut deserializer)?;
// //     Ok(t)
// // }

// type Result<T> = CoreResult<T, crate::Error>;

// struct SubEntryDeserializer<'de> {
//     iter: SubEntryIter<'de>,
//     value: Option<&'de str>
// }

// impl<'de> SubEntryDeserializer<'de> {
//     fn new(subentry: &'de SubEntry<'de>) -> Self {
//         SubEntryDeserializer {
//             iter: subentry.iter(),
//             value: None,
//         }
//     }
// }


// impl<'de> MapAccess<'de> for SubEntryDeserializer<'de> {
//     type Error = crate::Error;

//     fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
//     where
//         T: DeserializeSeed<'de>,
//     {
//         match self.iter.next() {
//             Some((key, value)) => {
//                 self.value = Some(value);
//                 Ok(None)
//                 // let key_de = MapKeyDeserializer {
//                 //     key: Cow::Owned(key),
//                 // };
//                 // seed.deserialize(key_de).map(Some)
//             }
//             None => Ok(None),
//         }
//     }

//     fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value>
//     where
//         T: DeserializeSeed<'de>,
//     {
//         match self.value.take() {
//             Some(value) => seed.deserialize(value),
//             None => Err(crate::Error::SerdeError("value is missing".to_string())),
//         }
//     }

//     fn size_hint(&self) -> Option<usize> {
//         match self.iter.size_hint() {
//             (lower, Some(upper)) if lower == upper => Some(upper),
//             _ => None,
//         }
//     }
// }

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