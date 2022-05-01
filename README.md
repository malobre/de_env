# 🦀 `de_env` &emsp;

**De**serialize **env**ironment variables through serde.

---

[Documentation](https://docs.rs/de_env) | [Crates.io](https://crates.io/crates/de_env)

## Example

Assuming we have a `LOG` and `PORT` environment variable:

```rust,no_run
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Config {
    log: String,
    port: u16
}

let config: Config = de_env::from_env().unwrap();

println!("{config:#?}");
```

## Boolean parsing

**Boolean deserialization is case-insensitive.**

If the `truthy-falsy` feature is enabled (default):

- Truthy values:
  - `true` or its shorthand `t`
  - `yes` or its shorthand `y`
  - `on`
  - `1`
- Falsy values:
  - `false` or its shorthand `f`
  - `no` or its shorthand `n`
  - `off`
  - `0`

If the `truthy-falsy` feature is disabled, only `true` and `false` are
considered valid booleans.

## Enum

**Only unit variants can be deserialized.**

Assuming we have a `LEVEL` environment variable set to `HIGH`, `MEDIUM` or
`LOW`:

```rust,no_run
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Level {
    High,
    Medium,
    Low
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Pool {
    level: Level,
}

let pool: Pool = de_env::from_env().unwrap();

println!("{pool:#?}");
```

## Unsupported types

- Nested structs
- Nested enums
- Nested Maps
- Non-unit enum variants
- Tuples
- Sequences
- Byte Arrays
