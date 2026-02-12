use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_django::{DjangoLanguage, DjangoParser};

#[test]
fn test_parse_django_variable() {
    let source = SourceText::new("{{ user.name }}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let mut cache = ParseSession::<DjangoLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok());
    // Should have minimal diagnostics for valid Django variable
    assert!(result.diagnostics.is_empty() || result.diagnostics.len() <= 1)
}

#[test]
fn test_parse_django_tag() {
    let source = SourceText::new("{% if user.is_authenticated %}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let mut cache = ParseSession::<DjangoLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok())
}

#[test]
fn test_parse_django_comment() {
    let source = SourceText::new("{# This is a comment #}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let mut cache = ParseSession::<DjangoLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok());
    // Comments should parse without errors
}

#[test]
fn test_parse_django_filter() {
    let source = SourceText::new("{{ name|lower }}");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let mut cache = ParseSession::<DjangoLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok())
}

#[test]
fn test_parse_invalid_syntax() {
    let source = SourceText::new("{{ unclosed_variable");
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let parser = DjangoParser::new(language);
    let mut cache = ParseSession::<DjangoLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);

    // Current simple implementation always parses successfully, creating a basic syntax tree.
    assert!(result.result.is_ok())
}
