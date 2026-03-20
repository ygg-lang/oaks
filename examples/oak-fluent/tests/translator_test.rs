use oak_fluent::{Translator, parse};
use std::collections::HashMap;

#[test]
fn test_basic_translation() {
    let ftl = r#"hello = Hello world!
welcome = Welcome to oak-fluent!"#;

    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);

    let args = HashMap::new();
    assert_eq!(translator.translate("hello", &args), Some("Hello world!".to_string()));
    assert_eq!(translator.translate("welcome", &args), Some("Welcome to oak-fluent!".to_string()));
    assert_eq!(translator.translate("non_existent", &args), None);
}

#[test]
fn test_translation_with_args() {
    let ftl = r#"greeting = Hello, { $name }!
item_count = You have { $count } items."#;

    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);

    let mut args = HashMap::new();
    args.insert("name".to_string(), "John".to_string());
    assert_eq!(translator.translate("greeting", &args), Some("Hello, John!".to_string()));

    args.clear();
    args.insert("count".to_string(), "5".to_string());
    assert_eq!(translator.translate("item_count", &args), Some("You have 5 items.".to_string()));
}

#[test]
fn test_translation_cache() {
    let ftl = r#"greeting = Hello, { $name }!"#;

    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);

    // First call - should cache the result
    let mut args = HashMap::new();
    args.insert("name".to_string(), "John".to_string());
    assert_eq!(translator.translate("greeting", &args), Some("Hello, John!".to_string()));

    // Second call - should use cached result
    assert_eq!(translator.translate("greeting", &args), Some("Hello, John!".to_string()));
}

#[test]
fn test_translation_with_different_args_order() {
    let ftl = r#"greeting = Hello, { $name }! You have { $count } items."#;

    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);

    // First call with args in order name, count
    let mut args1 = HashMap::new();
    args1.insert("name".to_string(), "John".to_string());
    args1.insert("count".to_string(), "5".to_string());
    assert_eq!(translator.translate("greeting", &args1), Some("Hello, John! You have 5 items.".to_string()));

    // Second call with args in order count, name
    let mut args2 = HashMap::new();
    args2.insert("count".to_string(), "5".to_string());
    args2.insert("name".to_string(), "John".to_string());
    assert_eq!(translator.translate("greeting", &args2), Some("Hello, John! You have 5 items.".to_string()));
}
