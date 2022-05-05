use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

use serde::de::IntoDeserializer;

use crate::{Error, Result};

pub struct Value<'de>(Cow<'de, OsStr>);

impl<'de> From<Cow<'de, OsStr>> for Value<'de> {
    fn from(value: Cow<'de, OsStr>) -> Self {
        Self(value)
    }
}

impl<'de> From<&'de OsStr> for Value<'de> {
    fn from(value: &'de OsStr) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'de> From<OsString> for Value<'de> {
    fn from(value: OsString) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'de> From<Cow<'de, str>> for Value<'de> {
    fn from(value: Cow<'de, str>) -> Self {
        match value {
            Cow::Owned(string) => Self(Cow::Owned(OsString::from(string))),
            Cow::Borrowed(str) => Self(Cow::Borrowed(OsStr::new(str))),
        }
    }
}

impl<'de> From<&'de str> for Value<'de> {
    fn from(value: &'de str) -> Self {
        Self(Cow::Borrowed(OsStr::new(value)))
    }
}

impl<'de> From<String> for Value<'de> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(OsString::from(value)))
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! validate_unicode_and_parse {
    ($($ty:ident)*) => {
        paste::paste! {
            $(
                fn [<deserialize_ $ty>]<V>(self, visitor: V) -> Result<V::Value>
                where
                    V: serde::de::Visitor<'de>
                {
                    match self.0.to_str() {
                        Some(str) => visitor.[<visit_ $ty>](str.parse::<$ty>()?),
                        None => Err(Error::invalid_unicode(self.0.into_owned())),
                    }
                }
            )*
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for Value<'de> {
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

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let lowercase_input = self.0.to_str().map(str::to_lowercase);

        #[cfg(feature = "truthy-falsy")]
        match lowercase_input.as_deref() {
            Some("true" | "t" | "yes" | "y" | "on" | "1") => visitor.visit_bool(true),
            Some("false" | "f" | "no" | "n" | "off" | "0") => visitor.visit_bool(false),
            _ => Err(Error::invalid_bool(self.0.into_owned())),
        }

        #[cfg(not(feature = "truthy-falsy"))]
        match lowercase_input.as_deref() {
            Some("true") => visitor.visit_bool(true),
            Some("false") => visitor.visit_bool(false),
            _ => Err(Error::invalid_bool(self.0.into_owned())),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.0.to_str() {
            Some(str) => str
                .into_deserializer()
                .deserialize_enum(name, variants, visitor),
            None => Err(Error::invalid_unicode(self.0.into_owned())),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    validate_unicode_and_parse! {
        u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
    }

    crate::de::util::unsupported_types! {
        bytes byte_buf unit unit_struct seq tuple
        tuple_struct map struct identifier any
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::Value;

    #[test]
    fn deserialize_bool() {
        #[cfg(feature = "truthy-falsy")]
        let truthy = [
            "true", "TRUE", "t", "T", "yes", "YES", "y", "Y", "on", "ON", "1",
        ];

        #[cfg(not(feature = "truthy-falsy"))]
        let truthy = ["true", "TRUE"];

        for value in truthy {
            assert!(matches!(bool::deserialize(Value::from(value)), Ok(true)));
        }

        #[cfg(feature = "truthy-falsy")]
        let falsy = [
            "false", "FALSE", "f", "F", "no", "NO", "n", "N", "off", "OFF", "0",
        ];

        #[cfg(not(feature = "truthy-falsy"))]
        let falsy = ["false", "FALSE"];

        for value in falsy {
            assert!(matches!(bool::deserialize(Value::from(value)), Ok(false)));
        }

        assert!(bool::deserialize(Value::from("gibberish")).is_err());
    }

    #[test]
    fn deserialize_enum() {
        #[derive(serde::Deserialize, Debug)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        enum Switch {
            On,
            Off,
            NewTypeVariant(bool),
            StructVariant { _field: bool },
        }

        assert!(matches!(
            Switch::deserialize(Value::from("ON")),
            Ok(Switch::On)
        ));

        assert!(matches!(
            Switch::deserialize(Value::from("OFF")),
            Ok(Switch::Off)
        ));

        assert!(Switch::deserialize(Value::from("NEW_TYPE_VARIANT")).is_err());
        assert!(Switch::deserialize(Value::from("STRUCT_VARIANT")).is_err());
        assert!(Switch::deserialize(Value::from("gibberish")).is_err());
    }

    #[test]
    fn deserialize_newtype_struct() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct NewType(u8);

        assert!(matches!(
            NewType::deserialize(Value::from("123")),
            Ok(NewType(123))
        ));
    }
}
