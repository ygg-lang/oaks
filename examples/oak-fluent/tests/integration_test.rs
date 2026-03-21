/// Integration tests for Oak Fluent.
use oak_fluent::parser::parse;
use oak_fluent::translator::Translator;
use std::collections::HashMap;

#[test]
fn test_basic_message_parsing() {
    let ftl = "greeting = Hello, world!";
    let result = parse(ftl).unwrap();
    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0].id, "greeting");
    assert!(result.messages[0].value.is_some());
}

#[test]
fn test_variable_reference() {
    let ftl = "greeting = Hello, { $name }!";
    let result = parse(ftl).unwrap();
    assert_eq!(result.messages.len(), 1);
    let pattern = result.messages[0].value.as_ref().unwrap();
    assert_eq!(pattern.elements.len(), 3); // Text, VariableReference, Text
}

#[test]
fn test_message_reference() {
    let ftl = "name = World\ngreeting = Hello, { name }!";
    let result = parse(ftl).unwrap();
    assert_eq!(result.messages.len(), 2);
    assert_eq!(result.messages[0].id, "name");
    assert_eq!(result.messages[1].id, "greeting");
}

#[test]
fn test_plural_form() {
    let ftl = "apple-count = { $count ->\n    [one] One apple\n   *[other] { $count } apples\n}";
    let result = parse(ftl).unwrap();
    assert_eq!(result.messages.len(), 1);
    let pattern = result.messages[0].value.as_ref().unwrap();
    assert_eq!(pattern.elements.len(), 1);
}

#[test]
fn test_translate_basic_message() {
    let ftl = "greeting = Hello, world!";
    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);
    let args = HashMap::new();
    let result = translator.translate("greeting", &args);
    assert_eq!(result, Some("Hello, world!".to_string()));
}

#[test]
fn test_translate_with_variables() {
    let ftl = "greeting = Hello, { $name }!";
    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);
    let mut args = HashMap::new();
    args.insert("name".to_string(), "Alice".to_string());
    let result = translator.translate("greeting", &args);
    assert_eq!(result, Some("Hello, Alice!".to_string()));
}

#[test]
fn test_translate_plural_form() {
    let ftl = "apple-count = { $count ->\n    [one] One apple\n   *[other] { $count } apples\n}";
    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);

    // Test singular form
    let mut args = HashMap::new();
    args.insert("count".to_string(), "1".to_string());
    let result = translator.translate("apple-count", &args);
    assert_eq!(result, Some("One apple".to_string()));

    // Test plural form
    args.insert("count".to_string(), "5".to_string());
    let result = translator.translate("apple-count", &args);
    assert_eq!(result, Some("5 apples".to_string()));
}

#[test]
fn test_translate_message_reference() {
    let ftl = "name = World\ngreeting = Hello, { name }!";
    let root = parse(ftl).unwrap();
    let translator = Translator::new(root);
    let args = HashMap::new();
    let result = translator.translate("greeting", &args);
    assert_eq!(result, Some("Hello, World!".to_string()));
}
