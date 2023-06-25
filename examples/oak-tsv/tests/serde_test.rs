#![cfg(feature = "serde")]
use oak_tsv::to_string;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[test]
fn test_serde_tsv() {
    let users = vec![User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() }];

    let tsv = to_string(&users).unwrap();
    println!("Serialized TSV:\n{}", tsv);

    // 1	Alice	alice@example.com

    // let deserialized: Vec<User> = from_str(&tsv).unwrap();
    // assert_eq!(users, deserialized);
}
