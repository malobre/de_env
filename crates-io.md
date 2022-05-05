`de_env` helps you easily **de**serialize **env**ironment variables to a struct,
a map or a seq (array, `Vec`, etc).

## Example

Assuming we have a `TIMEOUT`, `HOST` and `RETRY` environment variable:

```rust
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Config {
    timeout: u16,
    host: std::net::IpAddr,
    retry: bool,
}

let config: Config = de_env::from_env()?;

println!("{config:#?}");
```
