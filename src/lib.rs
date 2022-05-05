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
//! ```rust,no_run
//! #[derive(serde::Deserialize, Debug)]
//! #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
//! struct Config {
//!     timeout: u16,
//!     host: std::net::IpAddr,
//! }
//!
//! let config: Config = de_env::from_env()?;
//!
//! println!("{config:#?}");
//! # Ok::<(), de_env::Error>(())
//! ```
//!
//! ## Primitives
//!
//! ### Numbers
//!
//! If the input is valid Unicode, integers and floats are parsed with their
//! respective [`FromStr`](std::str::FromStr) implementations.
//!
//! ### Booleans
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
//! ```rust,no_run
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
//! let config: Config = de_env::from_env()?;
//!
//! println!("{config:#?}");
//! # Ok::<(), de_env::Error>(())
//! ```

mod de;
mod error;
#[cfg(test)]
mod tests;

pub use de::{from_env, from_env_prefixed, from_iter};
pub use error::{Error, Result};
