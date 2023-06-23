use oak_core::{SourceText, lexer::Lexer, parser::session::ParseSession};
use oak_rbq::{RbqLanguage, RbqLexer, RbqSyntaxKind};

#[test]
fn test_rbq_lexer_basic_tokens() {
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let source = SourceText::new("namespace App { struct User { id: i32 } }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check for expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&RbqSyntaxKind::NamespaceKw));
    assert!(token_kinds.contains(&RbqSyntaxKind::StructKw));
    assert!(token_kinds.contains(&RbqSyntaxKind::Ident));
    assert!(token_kinds.contains(&RbqSyntaxKind::LeftBrace));
    assert!(token_kinds.contains(&RbqSyntaxKind::RightBrace));
    assert!(token_kinds.contains(&RbqSyntaxKind::Colon))
}

#[test]
fn test_rbq_lexer_micro_keyword() {
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let source = SourceText::new("micro my_func() {}");

    let mut session = ParseSession::<RbqLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&RbqSyntaxKind::MicroKw))
}

#[test]
fn test_rbq_lexer_annotations() {
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let source = SourceText::new("@primary_key @schema(name=\"users\") struct User {}");

    let mut session = ParseSession::<RbqLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&RbqSyntaxKind::At));
    assert!(token_kinds.contains(&RbqSyntaxKind::StringLiteral))
}

#[test]
fn test_rbq_lexer_types() {
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let source = SourceText::new("List<String>?");

    let mut session = ParseSession::<RbqLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&RbqSyntaxKind::Lt));
    assert!(token_kinds.contains(&RbqSyntaxKind::Gt));
    assert!(token_kinds.contains(&RbqSyntaxKind::Question))
}

#[test]
fn test_rbq_lexer_utf8_keyword() {
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let source = SourceText::new("name: utf8");

    let mut session = ParseSession::<RbqLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&RbqSyntaxKind::Utf8Kw))
}
