#![allow(clippy::needless_doctest_main)]

use serde::{de::value::MapDeserializer, Deserialize};

use crate::{Error, Result};

use self::{key::Key, value::Value};

mod key;
mod util;
mod value;

/// Deserialize an instance of `T` from the environment variables of the current process.
///
/// # Example
/// Assuming we have a `LOG` and `PORT` environment variable:
/// ```rust
/// #[derive(serde::Deserialize, Debug)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// struct Config {
///     log: String,
///     port: u16
/// }
///
/// # std::env::set_var("LOG", "some value");
/// # std::env::set_var("PORT", "2345");
/// let config: Config = de_env::from_env()?;
///
/// println!("{config:#?}");
/// # Ok::<(), de_env::Error>(())
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
    from_iter(std::env::vars_os())
}

/// Deserialize an instance of `T` from the environment variables of the current process with the
/// specified prefix.
///
/// # Example
/// Assuming we have a `PREFIX_LOG` and `PREFIX_PORT` environment variable:
/// ```rust
/// #[derive(serde::Deserialize, Debug)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// struct Config {
///     log: String,
///     port: u16
/// }
///
/// # std::env::set_var("PREFIX_LOG", "some value");
/// # std::env::set_var("PREFIX_PORT", "2345");
/// let config: Config = de_env::from_env_prefixed("PREFIX_")?;
///
/// println!("{config:#?}");
/// # Ok::<(), de_env::Error>(())
/// ```
///
/// # Errors
/// This conversion can fail if trying to deserialize [unsupported types], or if `T`'s
/// implementation of `Deserialize` decides that something is wrong with the data.
///
/// [unsupported types]: crate#unsupported-types
pub fn from_env_prefixed<'de, T>(prefix: &str) -> Result<T>
where
    T: Deserialize<'de>,
{
    from_iter(
        std::env::vars_os().filter_map(|(name, value)| match name.to_str() {
            Some(name) => Some((std::ffi::OsString::from(name.strip_prefix(prefix)?), value)),
            _ => None,
        }),
    )
}

/// Deserialize an instance of `T` from an iterator of key-value tuple.
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
/// # std::env::set_var("LOG", "some value");
/// # std::env::set_var("PORT", "2345");
/// let vars = std::env::vars_os().filter(|(name, _value)| name == "LOG" || name == "PORT");
///
/// let config: Config = de_env::from_iter(vars)?;
///
/// println!("{config:#?}");
/// # Ok::<(), de_env::Error>(())
/// ```
///
/// # Errors
/// This conversion can fail if trying to deserialize [unsupported types], or if `T`'s
/// implementation of `Deserialize` decides that something is wrong with the data.
///
/// # Iterator Items
/// The items must be a tuple of 2 elements that are of the following types:
/// - [`OsString`](std::ffi::OsString)
/// - [`String`]
/// - [`Cow<str>`](std::borrow::Cow)
/// - [`Cow<OsStr>`](std::borrow::Cow)
/// - [`&OsStr`](std::ffi::OsStr)
/// - [`&str`]
///
/// [unsupported types]: crate#unsupported-types
pub fn from_iter<'de, T>(
    iter: impl Iterator<Item = (impl Into<Key<'de>>, impl Into<Value<'de>>)>,
) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer =
        EnvDeserializer::from_iter(iter.map(|(key, value)| (key.into(), value.into())));

    T::deserialize(&mut deserializer)
}

struct EnvDeserializer<'de, I: Iterator<Item = (Key<'de>, Value<'de>)>>(
    MapDeserializer<'de, I, Error>,
);

impl<'de, I> EnvDeserializer<'de, I>
where
    I: Iterator<Item = (Key<'de>, Value<'de>)>,
{
    pub fn from_iter(iter: I) -> Self {
        Self(MapDeserializer::new(iter))
    }
}

impl<'de, 'a, I> serde::de::Deserializer<'de> for &'a mut EnvDeserializer<'de, I>
where
    I: Iterator<Item = (Key<'de>, Value<'de>)>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(&mut self.0)
    }

    serde::forward_to_deserialize_any! {
        newtype_struct map struct enum ignored_any
    }

    util::unsupported_types! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct identifier
    }
}
