use dejavu_ast::*;
use dejavu_parser::parse;

#[test]
fn test_basic_template() {
    let template = parse(include_str!("basic.md.dejavu")).unwrap();

    // Check template mode
    assert_eq!(template.mode, TemplateMode::Html);

    // Check that we have elements
    assert!(!template.elements.is_empty());

    println!("Parsed template successfully: {:#?}", template);
}

#[test]
fn test_simple_expression() {
    let input = "<$ name $>";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::Variable(Variable(name)) => assert_eq!(name, "name"),
            _ => panic!("Expected variable expression"),
        },
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_text_element() {
    let input = "Hello World";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Text(text) => {
            assert_eq!(text.content, "Hello World");
        }
        _ => panic!("Expected text element"),
    }
}

#[test]
fn test_mixed_content() {
    let input = "Hello <$ name $>!";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 3);

    // First element should be text "Hello "
    match &template.elements[0] {
        DejavuElement::Text(text) => assert_eq!(text.content, "Hello "),
        _ => panic!("Expected text element"),
    }

    // Second element should be expression
    match &template.elements[1] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::Variable(Variable(name)) => assert_eq!(name, "name"),
            _ => panic!("Expected variable expression"),
        },
        _ => panic!("Expected expression element"),
    }

    // Third element should be text "!"
    match &template.elements[2] {
        DejavuElement::Text(text) => assert_eq!(text.content, "!"),
        _ => panic!("Expected text element"),
    }
}

#[test]
fn test_string_literal() {
    let input = r#"<$ "hello world" $>"#;
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::Literal(DejavuValue::String(s)) => assert_eq!(s.as_str(), "hello world"),
            _ => panic!("Expected string literal"),
        },
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_number_literal() {
    let input = "<$ 42 $>";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => {
            match &expr.term {
                Term::Literal(DejavuValue::Integer(_)) => {} // Just check it's an integer
                _ => panic!("Expected integer literal"),
            }
        }
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_boolean_literal() {
    let input = "<$ true $>";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::Literal(DejavuValue::Boolean(true)) => {}
            _ => panic!("Expected boolean literal true"),
        },
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_property_access() {
    let input = "<$ user.name $>";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::CallProperty(CallProperty { object, property }) => {
                match object.as_ref() {
                    Term::Variable(Variable(name)) => assert_eq!(name, "user"),
                    _ => panic!("Expected variable in property access"),
                }
                assert_eq!(property, "name");
            }
            _ => panic!("Expected property access"),
        },
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_method_call() {
    let input = "<$ items.len() $>";
    let template = parse(input).unwrap();

    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Expression(expr) => match &expr.term {
            Term::CallMethod(CallMethod { object, method, args }) => {
                match object.as_ref() {
                    Term::Variable(Variable(name)) => assert_eq!(name, "items"),
                    _ => panic!("Expected variable in method call"),
                }
                assert_eq!(method, "len");
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected method call"),
        },
        _ => panic!("Expected expression element"),
    }
}

#[test]
fn test_comments() {
    let input = "<# This is a comment #>Hello";
    let template = parse(input).unwrap();

    // Comments should be skipped, only text should remain
    assert_eq!(template.elements.len(), 1);
    match &template.elements[0] {
        DejavuElement::Text(text) => assert_eq!(text.content, "Hello"),
        _ => panic!("Expected text element"),
    }
}
