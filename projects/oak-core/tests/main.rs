#![feature(new_range_api)]

mod lexer;
mod parser;

#[test]
fn ready() {
    println!("it works!")
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_range_and_option() {
    use core::range::Range;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(with = "oak_core::serde_range")]
        pub range: Range<usize>,
        #[serde(with = "oak_core::serde_range::option")]
        pub opt_some: Option<Range<usize>>,
        #[serde(with = "oak_core::serde_range::option")]
        pub opt_none: Option<Range<usize>>,
    }

    let original = TestStruct { range: Range { start: 10, end: 20 }, opt_some: Some(Range { start: 30, end: 40 }), opt_none: None };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}
