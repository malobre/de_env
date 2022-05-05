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

    let test = crate::from_env::<Test>().unwrap();

    assert_eq!(test.a, "lorem ipsum");
    assert_eq!(test.b, 128);
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

    let _test: Test =
        crate::from_iter([("a", "12"), ("b", "34"), ("c", "56")].into_iter()).unwrap();
}

#[test]
fn option() {
    #[derive(serde::Deserialize)]
    struct Test {
        a: Option<u8>,
        b: Option<u8>,
    }

    let test: Test = crate::from_iter([("a", "12")].into_iter()).unwrap();

    assert_eq!(test.a, Some(12));
    assert_eq!(test.b, None);
}

#[test]
fn prefixed() {
    #[derive(serde::Deserialize, Debug)]
    struct Test {
        a: String,
        b: u8,
    }

    std::env::set_var("a", "wrong a");
    std::env::set_var("b", "wrong b");
    std::env::set_var("prefix_a", "lorem ipsum");
    std::env::set_var("prefix_b", "128");

    let test_struct = crate::from_env_prefixed::<Test>("prefix_").unwrap();

    assert_eq!(test_struct.a, "lorem ipsum");
    assert_eq!(test_struct.b, 128);
}
