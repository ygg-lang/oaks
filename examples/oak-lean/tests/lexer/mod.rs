use oak_lean::{LeanLexer, LeanSyntaxKind, LeanToken};
use oak_core::Lexer;

#[test]
fn test_keywords() {
    let input = "def theorem lemma axiom".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::Def,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Theorem,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Lemma,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Axiom,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_identifiers() {
    let input = "hello_world _private myVar".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::Identifier,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Identifier,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Identifier,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_numbers() {
    let input = "42 3.14 1.5e10".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::IntegerLiteral,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::FloatLiteral,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::FloatLiteral,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_strings() {
    let input = r#""hello world" "escaped \"quote\"" 'c'"#.to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::StringLiteral,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::StringLiteral,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::CharLiteral,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_comments() {
    let input = "-- line comment\n/- block comment -/".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::Comment,
        LeanSyntaxKind::Whitespace, // newline
        LeanSyntaxKind::Comment,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_operators() {
    let input = "+ - * / = != <= >= -> =>".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::Plus,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Minus,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Star,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Slash,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Eq,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Ne,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Le,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Ge,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Arrow,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::FatArrow,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_delimiters() {
    let input = "() {} [] : ; , .".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    let expected_kinds = vec![
        LeanSyntaxKind::LeftParen,
        LeanSyntaxKind::RightParen,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::LeftBrace,
        LeanSyntaxKind::RightBrace,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::LeftBracket,
        LeanSyntaxKind::RightBracket,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Colon,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Semicolon,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Comma,
        LeanSyntaxKind::Whitespace,
        LeanSyntaxKind::Dot,
        LeanSyntaxKind::Eof,
    ];

    assert_eq!(tokens.len(), expected_kinds.len());
    for (token, expected_kind) in tokens.iter().zip(expected_kinds.iter()) {
        assert_eq!(token.kind, *expected_kind);
    }
}

#[test]
fn test_complex_expression() {
    let input = "def factorial (n : Nat) : Nat := if n = 0 then 1 else n * factorial (n - 1)".to_string();
    let mut lexer = LeanLexer::new(input);
    let tokens = lexer.lex();

    // 验证第一个和最后一个标记
    assert_eq!(tokens[0].kind, LeanSyntaxKind::Def);
    assert_eq!(tokens[tokens.len() - 1].kind, LeanSyntaxKind::Eof);
    
    // 验证包含了预期的关键字
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&LeanSyntaxKind::Def));
    assert!(token_kinds.contains(&LeanSyntaxKind::If));
    assert!(token_kinds.contains(&LeanSyntaxKind::Then));
    assert!(token_kinds.contains(&LeanSyntaxKind::Else));
    assert!(token_kinds.contains(&LeanSyntaxKind::Identifier));
}
