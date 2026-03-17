use oak_yaml::language::{from_str, to_string};

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

    let yaml = to_string(&test_data).unwrap();
    println!("Serialized YAML:\n{}", yaml);

    let deserialized: TestStruct = from_str(&yaml).unwrap();
    assert_eq!(test_data, deserialized);
}

#[test]
fn test_serialize_primitive_types() {
    // Test string
    let s = "hello world";
    let yaml = to_string(&s).unwrap();
    println!("Serialized string: {}", yaml);
    let deserialized: String = from_str(&yaml).unwrap();
    assert_eq!(s, deserialized);

    // Test number
    let n = 42;
    let yaml = to_string(&n).unwrap();
    println!("Serialized number: {}", yaml);
    let deserialized: i32 = from_str(&yaml).unwrap();
    assert_eq!(n, deserialized);

    // Test boolean
    let b = true;
    let yaml = to_string(&b).unwrap();
    println!("Serialized boolean: {}", yaml);
    let deserialized: bool = from_str(&yaml).unwrap();
    assert_eq!(b, deserialized);
}

#[test]
fn test_serialize_collections() {
    // Test vector
    let v = vec![1, 2, 3, 4, 5];
    let yaml = to_string(&v).unwrap();
    println!("Serialized vector:\n{}", yaml);
    let deserialized: Vec<i32> = from_str(&yaml).unwrap();
    assert_eq!(v, deserialized);

    // Test map
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);
    let yaml = to_string(&map).unwrap();
    println!("Serialized map:\n{}", yaml);
    let deserialized: std::collections::HashMap<String, i32> = from_str(&yaml).unwrap();
    assert_eq!(map, deserialized);
}
