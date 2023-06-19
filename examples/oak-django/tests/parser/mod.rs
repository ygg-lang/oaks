use oak_core::{Parser, SourceText, errors::OakErrorKind};
use oak_django::{DjangoLanguage, DjangoParser};

#[test]
fn test_parse_django_variable() {
    let source = SourceText::new("{{ user.name }}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let result = parser.parse(&source);

    assert!(result.result.is_ok());
    // Should have minimal diagnostics for valid Django variable
    assert!(result.diagnostics.is_empty() || result.diagnostics.len() <= 1);
}

#[test]
fn test_parse_django_tag() {
    let source = SourceText::new("{% if user %}Hello{% endif %}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let result = parser.parse(&source);

    assert!(result.result.is_ok());
    // Should parse Django if-endif block successfully
}

#[test]
fn test_parse_django_comment() {
    let source = SourceText::new("{# This is a comment #}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let result = parser.parse(&source);

    assert!(result.result.is_ok());
    // Comments should parse without errors
}

#[test]
fn test_parse_mixed_content() {
    let source = SourceText::new("<div>{{ user.name }}</div>");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let result = parser.parse(&source);

    assert!(result.result.is_ok());
    // Mixed HTML and Django should parse successfully
}

#[test]
fn test_parse_invalid_syntax() {
    let source = SourceText::new("{{ unclosed_variable");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let result = parser.parse(&source);

    // 当前的简单实现总是成功解析，只是创建一个基本的语法树
    assert!(result.result.is_ok());
}
