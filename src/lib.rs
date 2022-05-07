//! # `de_env`
//!
//! _**De**serialize **env**ironment variables into a struct._
//!
//! ---
//!
//! You may be looking for:
//!
//! - [Git repository](https://github.com/malobre/de_env)
//! - [Crates.io](https://crates.io/crates/de_env)
//!
//! ## Example
//!
//! Assuming we have a `TIMEOUT` and `HOST` environment variable:
//!
//! ```rust
//! #[derive(serde::Deserialize, Debug)]
//! #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
//! struct Config {
//!     timeout: u16,
//!     host: std::net::IpAddr,
//! }
//!
//! # std::env::set_var("TIMEOUT", "12");
//! # std::env::set_var("HOST", "127.0.0.1");
//! let config: Config = de_env::from_env()?;
//!
//! println!("{config:#?}");
//! # Ok::<(), de_env::Error>(())
//! ```
//!
//! ## Supported Primitives
//!
//! - String slices
//! - Chars
//! - Numbers (parsed with their respective [`FromStr`](std::str::FromStr) implementations)
//! - Booleans (see [boolean parsing](#boolean-parsing))
//!
//! ## Boolean Parsing
//!
//! **Boolean parsing is case-insensitive.**
//!
//! If the `truthy-falsy` feature is enabled (default):
//!
//! - Truthy values:
//!   - `true` or its shorthand `t`
//!   - `yes` or its shorthand `y`
//!   - `on`
//!   - `1`
//! - Falsy values:
//!   - `false` or its shorthand `f`
//!   - `no` or its shorthand `n`
//!   - `off`
//!   - `0`
//!
//! If the `truthy-falsy` feature is disabled, only `true` and `false` are
//! considered valid booleans.
//!
//! ## Enums
//!
//! **Only unit variants can be deserialized.**
//!
//! Assuming we have a `LOG_LEVEL` environment variable set to `INFO` or `WARN`:
//!
//! ```rust
//! #[derive(serde::Deserialize, Debug)]
//! #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
//! enum Level {
//!     Info,
//!     Warn
//! }
//!
//! #[derive(serde::Deserialize, Debug)]
//! #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
//! struct Config {
//!     log_level: Level,
//! }
//!
//! # std::env::set_var("LOG_LEVEL", "INFO");
//! let config: Config = de_env::from_env()?;
//!
//! println!("{config:#?}");
//! # Ok::<(), de_env::Error>(())
//! ```
//!
//! ## Unsupported Types
//!
//! The goal of this crate is to deserialize environment variables into a **struct**, no other type
//! is supported at top level. Custom types must be able to deserialize from [supported primitives].
//!
//! [supported primitives]: #supported-primitives

mod de;
mod error;
#[cfg(test)]
mod tests;

pub use de::{from_env, from_env_prefixed, from_iter};
pub use error::{Error, Result};
