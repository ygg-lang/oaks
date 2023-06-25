mod lexer;

use oak_core::{Lexer, ParseSession, Parser, SourceText};
use oak_twig::{TwigLanguage, kind::TwigSyntaxKind};

#[test]
fn test_lexer_basic() {
    let language = TwigLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("{{ variable }}");
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len())
}

#[test]
fn test_parser_basic() {
    let language = TwigLanguage::new();
    let parser = language.parser();
    let source = SourceText::new("{{ variable }}");
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    println!("Parsed tree with {} children", tree.children.len())
}

#[test]
fn test_lexer_string() {
    let language = TwigLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new(r#"{{ "hello world" }}"#);
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check if string kind exists.
    let has_string = tokens.iter().any(|t| matches!(t.kind, TwigSyntaxKind::String));
    assert!(has_string, "Should contain a string token")
}

#[test]
fn test_lexer_number() {
    let language = TwigLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("{{ 123 }}");
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check if number token exists.
    let has_number = tokens.iter().any(|t| matches!(t.kind, TwigSyntaxKind::Number));
    assert!(has_number, "Should contain a number token")
}

#[test]
fn test_lexer_boolean() {
    let language = TwigLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("{{ true }}");
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check if boolean token exists.
    let has_boolean = tokens.iter().any(|t| matches!(t.kind, TwigSyntaxKind::Boolean));
    assert!(has_boolean, "Should contain a boolean token")
}

#[test]
fn test_parser_variable() {
    let language = TwigLanguage::new();
    let parser = language.parser();
    let source = SourceText::new("{{ name }}");
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_parser_block() {
    let language = TwigLanguage::new();
    let parser = language.parser();
    let source = SourceText::new(
        r#"{% if condition %}
    Hello World
{% endif %}"#,
    );
    let mut session = ParseSession::<TwigLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_empty_input() {
    let language = TwigLanguage::new();
    let lexer = language.lexer();
    let parser = language.parser();
    let source = SourceText::new("");
    let mut session = ParseSession::<TwigLanguage>::default();

    // Test lexing of empty input
    let lex_result = lexer.lex(&source, &[], &mut session);
    assert!(lex_result.result.is_ok());

    // Test parsing of empty input
    let parse_result = parser.parse(&source, &[], &mut session);
    assert!(parse_result.result.is_ok())
}
