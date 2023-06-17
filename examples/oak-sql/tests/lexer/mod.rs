use oak_core::{SourceText, errors::OakErrorKind};
use oak_json::{language::JsonLanguage, lexer::JsonLexer};

#[test]
fn lex_unterminated_string_reports_correct_location() {
    // line 1: text, line 2 starts with an unterminated string
    let source = SourceText::new("line1\n\"abc");
    let binding = JsonLanguage::standard();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    // Expect a kind error for unterminated string starting at the quote on line 2, column 0
    let err = diagnostics
        .diagnostics
        .into_iter()
        .find(|e| matches!(e.kind(), OakErrorKind::SyntaxError { message, .. } if message.contains("Unterminated")))
        .expect("expected Unterminated string kind error");
    let loc = match err.kind() {
        OakErrorKind::SyntaxError { source, .. } => source,
        _ => unreachable!(),
    };
    assert_eq!(loc.line, 2);
    assert_eq!(loc.column, 0);
}

#[test]
fn lex_invalid_number_exponent_location() {
    let source = SourceText::new("1e+");
    let binding = JsonLanguage::standard();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    let err = diagnostics
        .diagnostics
        .into_iter()
        .find(|e| matches!(e.kind(), OakErrorKind::SyntaxError { message, .. } if message.contains("Invalid number")))
        .expect("expected Invalid number error");
    let loc = match err.kind() {
        OakErrorKind::SyntaxError { source, .. } => source,
        _ => unreachable!(),
    };
    assert_eq!((loc.line, loc.column), (1, 0));
}

#[test]
fn lex_unterminated_block_comment_location_json5() {
    let source = SourceText::new("/* unterminated");
    let binding = JsonLanguage::json5();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    let err = diagnostics
        .diagnostics
        .into_iter()
        .find(|e| matches!(e.kind(), OakErrorKind::SyntaxError { message, .. } if message.contains("Unterminated comment")))
        .expect("expected Unterminated comment kind error");
    let loc = match err.kind() {
        OakErrorKind::SyntaxError { source, .. } => source,
        _ => unreachable!(),
    };
    assert_eq!(loc.line, 1);
    assert_eq!(loc.column, 0); // starts at the '/'
}

#[test]
fn lex_basic_tokens() {
    let source = SourceText::new(r#"{"key": 123, "array": [true, false, null]}"#);
    let binding = JsonLanguage::standard();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    assert!(diagnostics.result.is_ok());
    assert!(diagnostics.diagnostics.is_empty());

    let tokens = diagnostics.result.unwrap();
    assert!(!tokens.is_empty());

    // 验证第一token 是左大括
    assert_eq!(tokens[0].kind, oak_json::JsonSyntaxKind::LeftBrace);
}

#[test]
fn lex_json5_features() {
    let source = SourceText::new(
        r#"{key: 'single quotes', // comment
    trailing: 'comma',}"#,
    );
    let binding = JsonLanguage::json5();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    assert!(diagnostics.result.is_ok());
    let tokens = diagnostics.result.unwrap();

    // 应该包含注释 kind
    assert!(tokens.iter().any(|t| t.kind == oak_json::JsonSyntaxKind::Comment));
}

#[test]
fn lex_hex_numbers() {
    let source = SourceText::new("0xFF");
    let binding = JsonLanguage::json5();
    let lexer = JsonLexer::new(&binding);
    let diagnostics = lexer.tokenize_source(&source);

    assert!(diagnostics.result.is_ok());
    let tokens = diagnostics.result.unwrap();

    // 应该识别为数
    assert!(tokens.iter().any(|t| t.kind == oak_json::JsonSyntaxKind::Number));
}
