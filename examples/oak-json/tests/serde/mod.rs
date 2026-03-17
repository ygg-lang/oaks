use oak_json::language::{from_str, to_string};

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

#[test]
fn test_serialize_struct() {
    let test_data =
        TestStruct { name: "John Doe".to_string(), age: 30, hobbies: vec!["reading".to_string(), "hiking".to_string(), "coding".to_string()], address: Address { street: "123 Main St".to_string(), city: "New York".to_string(), zip: "10001".to_string() } };

    let json = to_string(&test_data).unwrap();
    println!("Serialized JSON:\n{}", json);

    let deserialized: TestStruct = from_str(&json).unwrap();
    assert_eq!(test_data, deserialized);
}

#[test]
fn test_serialize_primitive_types() {
    // Test string
    let s = "hello world";
    let json = to_string(&s).unwrap();
    println!("Serialized string: {}", json);
    let deserialized: String = from_str(&json).unwrap();
    assert_eq!(s, deserialized);

    // Test number
    let n = 42;
    let json = to_string(&n).unwrap();
    println!("Serialized number: {}", json);
    let deserialized: i32 = from_str(&json).unwrap();
    assert_eq!(n, deserialized);

    // Test boolean
    let b = true;
    let json = to_string(&b).unwrap();
    println!("Serialized boolean: {}", json);
    let deserialized: bool = from_str(&json).unwrap();
    assert_eq!(b, deserialized);
}

#[test]
fn test_serialize_collections() {
    // Test vector
    let v = vec![1, 2, 3, 4, 5];
    let json = to_string(&v).unwrap();
    println!("Serialized vector:\n{}", json);
    let deserialized: Vec<i32> = from_str(&json).unwrap();
    assert_eq!(v, deserialized);

    // Test map
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);
    let json = to_string(&map).unwrap();
    println!("Serialized map:\n{}", json);
    let deserialized: std::collections::HashMap<String, i32> = from_str(&json).unwrap();
    assert_eq!(map, deserialized);
}
