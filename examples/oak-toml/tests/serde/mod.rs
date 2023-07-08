use oak_toml::language::{from_str, to_string};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct TestStruct {
    name: String,
    age: u32,
    hobbies: Vec<String>,
    address: Address,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[ignore = "TOML serde round-trip deserialization is incomplete"]
#[test]
fn test_serialize_struct() {
    let test_data =
        TestStruct { name: "John Doe".to_string(), age: 30, hobbies: vec!["reading".to_string(), "hiking".to_string(), "coding".to_string()], address: Address { street: "123 Main St".to_string(), city: "New York".to_string(), zip: "10001".to_string() } };

    let toml = to_string(&test_data).unwrap();
    println!("Serialized TOML:\n{}", toml);

    let deserialized: TestStruct = from_str(&toml).unwrap();
    assert_eq!(test_data, deserialized);
}

#[ignore = "TOML serde round-trip deserialization is incomplete"]
#[test]
fn test_serialize_primitive_types() {
    // Test string
    let s = "hello world";
    let toml = to_string(&s).unwrap();
    println!("Serialized string: {}", toml);
    let deserialized: String = from_str(&toml).unwrap();
    assert_eq!(s, deserialized);

    // Test number
    let n = 42;
    let toml = to_string(&n).unwrap();
    println!("Serialized number: {}", toml);
    let deserialized: i32 = from_str(&toml).unwrap();
    assert_eq!(n, deserialized);

    // Test boolean
    let b = true;
    let toml = to_string(&b).unwrap();
    println!("Serialized boolean: {}", toml);
    let deserialized: bool = from_str(&toml).unwrap();
    assert_eq!(b, deserialized);
}

#[ignore = "TOML serde round-trip deserialization is incomplete"]
#[test]
fn test_serialize_collections() {
    // Test vector
    let v = vec![1, 2, 3, 4, 5];
    let toml = to_string(&v).unwrap();
    println!("Serialized vector:\n{}", toml);
    let deserialized: Vec<i32> = from_str(&toml).unwrap();
    assert_eq!(v, deserialized);

    // Test map
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);
    let toml = to_string(&map).unwrap();
    println!("Serialized map:\n{}", toml);
    let deserialized: std::collections::HashMap<String, i32> = from_str(&toml).unwrap();
    assert_eq!(map, deserialized);
}
