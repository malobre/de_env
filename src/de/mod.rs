#![allow(clippy::needless_doctest_main)]

use std::ffi::OsString;

use serde::{de::value::MapDeserializer, Deserialize};

use crate::{Error, Result};

use self::{key::Key, value::Value};

mod key;
mod value;

/// Deserialize an instance of `T` from the environment variables of the current process.
///
/// # Example
/// _Note: This assumes that you have set the specified environment variables._
/// ```rust
/// #[derive(serde::Deserialize, Debug)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// struct Config {
///     log: String,
///     port: u16
/// }
///
/// fn main() {
/// #   std::env::set_var("LOG", "some value");
/// #   std::env::set_var("PORT", "2345");
///     let config: Config = de_env::from_env().unwrap();
///
///     println!("{config:#?}");
/// }
/// ```
///
/// # Errors
/// This conversion can fail if trying to deserialize [unsupported types], or if `T`'s
/// implementation of `Deserialize` decides that something is wrong with the data.
///
/// [unsupported types]: crate#unsupported-types
pub fn from_env<'de, T>() -> Result<T>
where
    T: Deserialize<'de>,
{
    from_iter_os(std::env::vars_os())
}

/// Deserialize an instance of `T` from an iterator of name-value `String` pairs.
///
/// # Example
/// ```rust
/// #[derive(serde::Deserialize, Debug)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// struct Config {
///     log: String,
///     port: u16
/// }
///
/// fn main() {
/// #   std::env::set_var("LOG", "some value");
/// #   std::env::set_var("PORT", "2345");
///     // You can use any iterator over a `(String, String)` tuple.
///     let vars = std::env::vars().filter(|(name, _value)| name == "LOG" || name == "PORT");
///
///     let config: Config = de_env::from_iter(vars).unwrap();
///
///     println!("{config:#?}");
/// }
/// ```
///
/// # Errors
/// This conversion can fail if trying to deserialize [unsupported types], or if `T`'s
/// implementation of `Deserialize` decides that something is wrong with the data.
///
/// [unsupported types]: crate#unsupported-types
pub fn from_iter<'de, T>(iter: impl Iterator<Item = (String, String)>) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = EnvDeserializer::from_iter(iter.map(|(key, value)| {
        (
            Key::from(OsString::from(key)),
            Value::from(OsString::from(value)),
        )
    }));

    T::deserialize(&mut deserializer)
}

/// Deserialize an instance of `T` from an iterator of name-value `OsString` pairs.
///
/// # Example
/// ```rust
/// #[derive(serde::Deserialize, Debug)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// struct Config {
///     log: String,
///     port: u16
/// }
///
/// fn main() {
/// #   std::env::set_var("LOG", "some value");
/// #   std::env::set_var("PORT", "2345");
///     // You can use any iterator over a `(OsString, OsString)` tuple.
///     let vars = std::env::vars_os().filter(|(name, _value)| name == "LOG" || name == "PORT");
///
///     let config: Config = de_env::from_iter_os(vars).unwrap();
///
///     println!("{config:#?}");
/// }
/// ```
///
/// # Errors
/// This conversion can fail if trying to deserialize [unsupported types], or if `T`'s
/// implementation of `Deserialize` decides that something is wrong with the data.
///
/// [unsupported types]: crate#unsupported-types
pub fn from_iter_os<'de, T>(iter: impl Iterator<Item = (OsString, OsString)>) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer =
        EnvDeserializer::from_iter(iter.map(|(key, value)| (Key::from(key), Value::from(value))));

    T::deserialize(&mut deserializer)
}

struct EnvDeserializer<'de, I: Iterator<Item = (Key, Value)>>(MapDeserializer<'de, I, Error>);

impl<'de, I> EnvDeserializer<'de, I>
where
    I: Iterator<Item = (Key, Value)>,
{
    pub fn from_iter(iter: I) -> Self {
        Self(MapDeserializer::new(iter))
    }
}

impl<'de, 'a, I> serde::de::Deserializer<'de> for &'a mut EnvDeserializer<'de, I>
where
    I: Iterator<Item = (Key, Value)>,
{
    type Error = Error;

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(&mut self.0)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
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
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        unit seq
        bytes byte_buf
        unit_struct tuple_struct
        identifier tuple ignored_any option
    }
}
