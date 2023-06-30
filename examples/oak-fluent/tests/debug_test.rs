/// Debug tests for Oak Fluent parsing and translation.
use oak_fluent::parser::parse;
use oak_fluent::translator::Translator;
use std::collections::HashMap;

#[test]
fn debug_variable_reference() {
    let ftl = "greeting = Hello, { $name }!";
    let result = parse(ftl).unwrap();
    println!("Messages: {:?}", result.messages);
    let pattern = result.messages[0].value.as_ref().unwrap();
    println!("Pattern elements: {:?}", pattern.elements);
    println!("Number of elements: {}", pattern.elements.len());
}

#[test]
fn debug_plural_form() {
    let ftl = "apple-count = { $count ->\n    [one] One apple\n   *[other] { $count } apples\n}";
    let result = parse(ftl).unwrap();
    println!("Messages: {:?}", result.messages);
    let pattern = result.messages[0].value.as_ref().unwrap();
    println!("Pattern elements: {:?}", pattern.elements);

    // Test translation
    let translator = Translator::new(result);
    let mut args = HashMap::new();
    args.insert("count".to_string(), "5".to_string());
    let result = translator.translate("apple-count", &args);
    println!("Translation result: {:?}", result);
}
