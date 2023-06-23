use oak_core::{SourceText, lexer::Lexer, parser::session::ParseSession};
use oak_sql::{SqlLanguage, SqlLexer, SqlSyntaxKind};

#[test]
fn test_sql_lexer_basic_tokens() {
    let config = SqlLanguage::default();
    let lexer = SqlLexer::new(&config);
    let source = SourceText::new("SELECT * FROM users WHERE id = 42;");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check for expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&SqlSyntaxKind::Select));
    assert!(token_kinds.contains(&SqlSyntaxKind::Star));
    assert!(token_kinds.contains(&SqlSyntaxKind::From));
    assert!(token_kinds.contains(&SqlSyntaxKind::Where));
    assert!(token_kinds.contains(&SqlSyntaxKind::Equal));
    assert!(token_kinds.contains(&SqlSyntaxKind::NumberLiteral));
    assert!(token_kinds.contains(&SqlSyntaxKind::Semicolon))
}

#[test]
fn test_sql_lexer_insert() {
    let config = SqlLanguage::default();
    let lexer = SqlLexer::new(&config);
    let source = SourceText::new("INSERT INTO users (name, age) VALUES ('John', 25);");

    let mut session = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&SqlSyntaxKind::Insert));
    assert!(token_kinds.contains(&SqlSyntaxKind::Into));
    assert!(token_kinds.contains(&SqlSyntaxKind::Values));
    assert!(token_kinds.contains(&SqlSyntaxKind::LeftParen));
    assert!(token_kinds.contains(&SqlSyntaxKind::RightParen))
}

#[test]
fn test_sql_lexer_keywords() {
    let config = SqlLanguage::default();
    let lexer = SqlLexer::new(&config);
    let source = SourceText::new("CREATE TABLE test (id INT PRIMARY KEY);");

    let mut session = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&SqlSyntaxKind::Create));
    assert!(token_kinds.contains(&SqlSyntaxKind::Table));
    assert!(token_kinds.contains(&SqlSyntaxKind::Int));
    assert!(token_kinds.contains(&SqlSyntaxKind::Primary));
    assert!(token_kinds.contains(&SqlSyntaxKind::Key))
}

#[test]
fn test_sql_lexer_strings() {
    let config = SqlLanguage::default();
    let lexer = SqlLexer::new(&config);
    let source = SourceText::new("SELECT 'hello world', \"quoted identifier\";");

    let mut session = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let string_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == SqlSyntaxKind::StringLiteral).collect();

    assert_eq!(string_tokens.len(), 2)
}
