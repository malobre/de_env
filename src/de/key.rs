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

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_string(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_identifier(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    crate::de::util::unsupported_types! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct map struct enum ignored_any any
    }
}
