use oak_core::{Lexer, ParseSession, Parser, SourceText};
use oak_tailwind::{language::TailwindLanguage, lexer::token_type::TailwindTokenType};

mod lexer;

#[test]
fn test_lexer_basic() {
    let language = TailwindLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("hover:bg-red-500 p-4 !m-2 [100px]");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len())
}

#[test]
fn test_parser_basic() {
    let language = TailwindLanguage::new();
    let parser = language.parser();
    let source = SourceText::new("hover:bg-red-500 p-4");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    println!("Parsed tree with {} children", tree.children.len())
}

#[test]
fn test_lexer_modifier() {
    let language = TailwindLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("hover: dark:");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();

    let has_modifier = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::Modifier));
    assert!(has_modifier, "Should contain a modifier token")
}

#[test]
fn test_lexer_utility() {
    let language = TailwindLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("bg-red-500 p-4");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();

    let has_utility = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::Utility));
    assert!(has_utility, "Should contain a utility token")
}

#[test]
fn test_lexer_arbitrary_value() {
    let language = TailwindLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("[100px] [#000]");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();

    let has_arb = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::ArbitraryValue));
    assert!(has_arb, "Should contain an arbitrary value token")
}

#[test]
fn test_parser_class() {
    let language = TailwindLanguage::new();
    let parser = language.parser();
    let source = SourceText::new("hover:bg-red-500");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_parser_directive() {
    let language = TailwindLanguage::new();
    let parser = language.parser();
    let source = SourceText::new("@tailwind base; @apply font-bold;");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_empty_input() {
    let language = TailwindLanguage::new();
    let lexer = language.lexer();
    let parser = language.parser();
    let source = SourceText::new("");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let lex_result = lexer.lex(&source, &[], &mut session);
    assert!(lex_result.result.is_ok());

    let parse_result = parser.parse(&source, &[], &mut session);
    assert!(parse_result.result.is_ok())
}
