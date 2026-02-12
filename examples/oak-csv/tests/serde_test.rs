#![cfg(feature = "serde")]
use oak_csv::{from_str, to_string};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[test]
fn test_serde_csv() {
    let users = vec![User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() }, User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() }];

    let csv = to_string(&users).unwrap();
    println!("Serialized CSV:\n{}", csv);

    // 1,Alice,alice@example.com
    // 2,Bob,bob@example.com

    let deserialized: Vec<User> = from_str(&csv).unwrap();
    assert_eq!(users, deserialized);
}
