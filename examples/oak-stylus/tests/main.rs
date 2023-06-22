mod lexer;

use oak_core::{Lexer, SourceText};
use oak_stylus::{kind::StylusSyntaxKind, language::StylusLanguage, lexer::StylusLexer};

#[test]
fn test_lexer_basic() {
    let language = StylusLanguage::new();
    let lexer = StylusLexer::new(&language);
    let source = SourceText::new("body\n  color red");

    let mut session = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len());
}

#[test]
fn test_lexer_identifiers() {
    let language = StylusLanguage::new();
    let lexer = StylusLexer::new(&language);
    let source = SourceText::new("body div p");

    let mut session = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 应该包含标识符 token
    let has_identifier = tokens.iter().any(|t| t.kind == StylusSyntaxKind::Identifier);
    assert!(has_identifier);
}

#[test]
fn test_lexer_numbers() {
    let language = StylusLanguage::new();
    let lexer = StylusLexer::new(&language);
    let source = SourceText::new("width 100px");

    let mut session = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 应该包含数字 token
    let has_number = tokens.iter().any(|t| t.kind == StylusSyntaxKind::Number);
    assert!(has_number);
}

#[test]
fn test_empty_input() {
    let language = StylusLanguage::new();
    let lexer = StylusLexer::new(&language);
    let source = SourceText::new("");

    let mut session = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    // 空输入应该只有 EOF token
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, StylusSyntaxKind::Eof);
}
