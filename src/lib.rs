#![doc = include_str!("../README.md")]

mod de;
mod error;

pub use de::{from_env, from_iter, from_iter_os};
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        #[derive(serde::Deserialize, Debug)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct Test {
            a: String,
            b: u8,
        }

        std::env::set_var("A", "lorem ipsum");
        std::env::set_var("B", "128");

        let test_struct = crate::from_env::<Test>().unwrap();

        assert_eq!(test_struct.a, "lorem ipsum");
        assert_eq!(test_struct.b, 128);
    }

    #[test]
    #[should_panic]
    fn deny_unknown_fields() {
        #[allow(dead_code)]
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Test {
            a: u8,
            b: u8,
        }

        let _test: Test = crate::from_iter(
            [
                (String::from("a"), String::from("12")),
                (String::from("b"), String::from("34")),
                (String::from("c"), String::from("56")),
            ]
            .into_iter(),
        )
        .unwrap();
    }
}
