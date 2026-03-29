use oak_von::from_str; // 使用 from_str 函数来反序列化 VON 字符串

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct TestStruct {
    optional_field: Option<String>,
    another_optional: Option<i32>,
}

#[test]
fn test_option_some() {
    // 测试 Some 变体
    let von_str = r#"{
        optional_field: Some("Hello"),
        another_optional: Some(42)
    }"#;

    let result: TestStruct = from_str(von_str).unwrap();
    assert_eq!(result, TestStruct { optional_field: Some("Hello".to_string()), another_optional: Some(42) });
}

#[test]
fn test_option_none() {
    // 测试 None 变体
    let von_str = r#"{
        optional_field: None,
        another_optional: None
    }"#;

    let result: TestStruct = from_str(von_str).unwrap();
    assert_eq!(result, TestStruct { optional_field: None, another_optional: None });
}
