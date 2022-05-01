use std::ffi::OsString;

use serde::de::IntoDeserializer;

use crate::{Error, Result};

pub struct Key(OsString);

impl From<OsString> for Key {
    fn from(value: OsString) -> Self {
        Self(value)
    }
}

impl IntoDeserializer<'_, Error> for Key {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::de::Deserializer<'de> for Key {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
