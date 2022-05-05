use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

use crate::{Error, Result};

pub struct Key<'de>(Cow<'de, OsStr>);

impl<'de> From<Cow<'de, OsStr>> for Key<'de> {
    fn from(value: Cow<'de, OsStr>) -> Self {
        Self(value)
    }
}

impl<'de> From<&'de OsStr> for Key<'de> {
    fn from(value: &'de OsStr) -> Self {
        Self::from(Cow::from(value))
    }
}

impl<'de> From<OsString> for Key<'de> {
    fn from(value: OsString) -> Self {
        Self::from(Cow::from(value))
    }
}

impl<'de> From<Cow<'de, str>> for Key<'de> {
    fn from(value: Cow<'de, str>) -> Self {
        match value {
            Cow::Owned(string) => Self(Cow::Owned(OsString::from(string))),
            Cow::Borrowed(str) => Self(Cow::Borrowed(OsStr::new(str))),
        }
    }
}

impl<'de> From<&'de str> for Key<'de> {
    fn from(value: &'de str) -> Self {
        Self(Cow::Borrowed(OsStr::new(value)))
    }
}

impl<'de> From<String> for Key<'de> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(OsString::from(value)))
    }
}

impl<'de> serde::de::IntoDeserializer<'de, Error> for Key<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::de::Deserializer<'de> for Key<'de> {
    type Error = Error;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.0.to_str() {
            Some(str) => visitor.visit_str(str),
            None => Err(Error::invalid_unicode(self.0.into_owned())),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(
            self.0
                .into_owned()
                .into_string()
                .map_err(Error::invalid_unicode)?,
        )
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
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
