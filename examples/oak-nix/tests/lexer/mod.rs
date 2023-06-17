use oak_core::SourceText;
use oak_nix::{NixLanguage, NixLexer, NixSyntaxKind};

#[test]
fn test_basic_identifier() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);
    let source = SourceText::new("hello");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 应该包含标识符和EOF kind
    assert_eq!(tokens[0].kind, NixSyntaxKind::Identifier);
    assert_eq!(tokens[tokens.len() - 1].kind, NixSyntaxKind::Eof);
}

#[test]
fn test_keywords() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    // 测试不同的关键字
    let test_cases = vec![
        ("let", NixSyntaxKind::Let),
        ("in", NixSyntaxKind::In),
        ("if", NixSyntaxKind::If),
        ("then", NixSyntaxKind::Then),
        ("else", NixSyntaxKind::Else),
        ("with", NixSyntaxKind::With),
        ("inherit", NixSyntaxKind::Inherit),
        ("rec", NixSyntaxKind::Rec),
    ];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_strings() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let test_cases = vec![("\"hello\"", NixSyntaxKind::String), ("''multiline string''", NixSyntaxKind::String)];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());

        let found = tokens.iter().any(|token| token.kind == expected_kind);
        assert!(found, "Expected to find {:?} in tokens for input: {}", expected_kind, input);
    }
}

#[test]
fn test_numbers() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let test_cases = vec!["42", "3.14", "0", "123456789"];

    for input in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let found = tokens.iter().any(|token| token.kind == NixSyntaxKind::Number);
        assert!(found, "Expected to find number token for input: {}", input);
    }
}

#[test]
fn test_comments() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let test_cases = vec!["# single line comment", "/* block comment */", "/* multi\n   line\n   comment */"];

    for input in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        let found = tokens.iter().any(|token| token.kind == NixSyntaxKind::Comment);
        assert!(found, "Expected to find comment token for input: {}", input);
    }
}

#[test]
fn test_operators() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let test_cases = vec![
        ("+", NixSyntaxKind::Plus),
        ("-", NixSyntaxKind::Minus),
        ("*", NixSyntaxKind::Star),
        ("/", NixSyntaxKind::Slash),
        ("==", NixSyntaxKind::EqualEqual),
        ("!=", NixSyntaxKind::NotEqual),
        ("<", NixSyntaxKind::Less),
        (">", NixSyntaxKind::Greater),
        ("<=", NixSyntaxKind::LessEqual),
        (">=", NixSyntaxKind::GreaterEqual),
        ("&&", NixSyntaxKind::AndAnd),
        ("||", NixSyntaxKind::OrOr),
        ("!", NixSyntaxKind::Bang),
        ("++", NixSyntaxKind::PlusPlus),
        ("//", NixSyntaxKind::Update),
    ];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_delimiters() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let test_cases = vec![
        ("(", NixSyntaxKind::LeftParen),
        (")", NixSyntaxKind::RightParen),
        ("[", NixSyntaxKind::LeftBracket),
        ("]", NixSyntaxKind::RightBracket),
        ("{", NixSyntaxKind::LeftBrace),
        ("}", NixSyntaxKind::RightBrace),
        (";", NixSyntaxKind::Semicolon),
        (":", NixSyntaxKind::Colon),
        (".", NixSyntaxKind::Dot),
        (",", NixSyntaxKind::Comma),
        ("=", NixSyntaxKind::Equal),
        ("?", NixSyntaxKind::Question),
        ("@", NixSyntaxKind::At),
    ];

    for (input, expected_kind) in test_cases {
        let source = SourceText::new(input);
        let result = lexer.lex(&source);
        assert!(result.result.is_ok());

        let tokens = result.result.unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_whitespace() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let source = SourceText::new("  \t  \n  ");
    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let has_whitespace = tokens.iter().any(|token| token.kind == NixSyntaxKind::Whitespace);
    let has_newline = tokens.iter().any(|token| token.kind == NixSyntaxKind::Newline);

    assert!(has_whitespace || has_newline);
}

#[test]
fn test_complex_expression() {
    let language = NixLanguage::new();
    let lexer = NixLexer::new(&language);

    let source = SourceText::new(
        r#"
let
  x = 42;
  y = "hello";
  z = { a = 1; b = 2; };
in
  x + y
"#,
    );

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    // 验证包含预期的token类型
    assert!(token_kinds.contains(&NixSyntaxKind::Let));
    assert!(token_kinds.contains(&NixSyntaxKind::In));
    assert!(token_kinds.contains(&NixSyntaxKind::Equal));
    assert!(token_kinds.contains(&NixSyntaxKind::Number));
    assert!(token_kinds.contains(&NixSyntaxKind::String));
    assert!(token_kinds.contains(&NixSyntaxKind::LeftBrace));
    assert!(token_kinds.contains(&NixSyntaxKind::RightBrace));
    assert!(token_kinds.contains(&NixSyntaxKind::Semicolon));
    assert!(token_kinds.contains(&NixSyntaxKind::Plus));
    assert!(token_kinds.contains(&NixSyntaxKind::Identifier));
    assert!(token_kinds.contains(&NixSyntaxKind::Eof));
}
