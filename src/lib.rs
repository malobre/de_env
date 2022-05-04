#![doc = include_str!("../README.md")]

mod de;
mod error;
#[cfg(test)]
mod tests;

pub use de::{from_env, from_env_prefixed, from_iter, from_iter_os};
pub use error::{Error, Result};
