use std::ffi::OsString;

use serde::de::IntoDeserializer;

use crate::{Error, Result};

pub struct Value(OsString);

impl From<OsString> for Value {
    fn from(value: OsString) -> Self {
        Self(value)
    }
}

impl IntoDeserializer<'_, Error> for Value {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! convert_into_string_and_parse {
    ($($ty:ident)*) => {
        paste::paste! {
            $(
                fn [<deserialize_ $ty>]<V>(self, visitor: V) -> Result<V::Value>
                where
                    V: serde::de::Visitor<'de>
                {
                    visitor.[<visit_ $ty>](
                        self.0
                            .into_string()
                            .map_err(Error::invalid_unicode)?
                            .parse::<$ty>()?
                    )
                }
            )*
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for Value {
    type Error = Error;

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

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_char(visitor)
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
            _ => Err(Error::invalid_bool(self.0)),
        }

        #[cfg(not(feature = "truthy-falsy"))]
        match lowercase_input.as_deref() {
            Some("true") => visitor.visit_bool(true),
            Some("false") => visitor.visit_bool(false),
            _ => Err(Error::invalid_bool(self.0)),
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
        self.0
            .into_string()
            .map_err(Error::invalid_unicode)?
            .into_deserializer()
            .deserialize_enum(name, variants, visitor)
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

    convert_into_string_and_parse! {
        u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
    }

    crate::de::util::unsupported_types! {
        bytes byte_buf unit unit_struct seq tuple
        tuple_struct map struct identifier any
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

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
            assert!(matches!(
                bool::deserialize(Value(OsString::from(value))),
                Ok(true)
            ));
        }

        #[cfg(feature = "truthy-falsy")]
        let falsy = [
            "false", "FALSE", "f", "F", "no", "NO", "n", "N", "off", "OFF", "0",
        ];

        #[cfg(not(feature = "truthy-falsy"))]
        let falsy = ["false", "FALSE"];

        for value in falsy {
            assert!(matches!(
                bool::deserialize(Value(OsString::from(value))),
                Ok(false)
            ));
        }

        assert!(bool::deserialize(Value(OsString::from("gibberish"))).is_err());
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
            Switch::deserialize(Value(OsString::from("ON"))),
            Ok(Switch::On)
        ));

        assert!(matches!(
            Switch::deserialize(Value(OsString::from("OFF"))),
            Ok(Switch::Off)
        ));

        assert!(Switch::deserialize(Value(OsString::from("NEW_TYPE_VARIANT"))).is_err());
        assert!(Switch::deserialize(Value(OsString::from("STRUCT_VARIANT"))).is_err());
        assert!(Switch::deserialize(Value(OsString::from("gibberish"))).is_err());
    }

    #[test]
    fn deserialize_newtype_struct() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct NewType(u8);

        assert!(matches!(
            NewType::deserialize(Value(OsString::from("123"))),
            Ok(NewType(123))
        ));
    }
}
