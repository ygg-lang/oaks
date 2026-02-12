#![cfg(feature = "serde")]
use oak_xml;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct User {
    #[serde(rename = "↯id")]
    id: u32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "email")]
    email: String,
}

#[test]
fn test_serde_xml_attrs() {
    let user = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
    let xml = oak_xml::to_string(&user).unwrap();
    println!("Serialized XML:\n{}", xml);

    let deserialized: User = oak_xml::from_str(&xml).unwrap();
    assert_eq!(user, deserialized);
}
