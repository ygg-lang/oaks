use oak_core::{SourceText, lexer::Lexer};
use oak_json::{JsonLanguage, JsonLexer, JsonSyntaxKind};

#[test]
fn test_json_lexer_basic_tokens() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"{"key": "value", "number": 42, "bool": true}"#);

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check for expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&JsonSyntaxKind::LeftBrace));
    assert!(token_kinds.contains(&JsonSyntaxKind::RightBrace));
    assert!(token_kinds.contains(&JsonSyntaxKind::String));
    assert!(token_kinds.contains(&JsonSyntaxKind::Colon));
    assert!(token_kinds.contains(&JsonSyntaxKind::Comma));
    assert!(token_kinds.contains(&JsonSyntaxKind::Number));
    assert!(token_kinds.contains(&JsonSyntaxKind::TrueKeyword));
}

#[test]
fn test_json_lexer_array() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"[1, 2, 3, "test"]"#);

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&JsonSyntaxKind::LeftBracket));
    assert!(token_kinds.contains(&JsonSyntaxKind::RightBracket));
    assert!(token_kinds.contains(&JsonSyntaxKind::Number));
    assert!(token_kinds.contains(&JsonSyntaxKind::String));
    assert!(token_kinds.contains(&JsonSyntaxKind::Comma));
}

#[test]
fn test_json_lexer_boolean_and_null() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"{"bool_true": true, "bool_false": false, "null_value": null}"#);

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&JsonSyntaxKind::TrueKeyword));
    assert!(token_kinds.contains(&JsonSyntaxKind::FalseKeyword));
    assert!(token_kinds.contains(&JsonSyntaxKind::NullKeyword));
}

#[test]
fn test_json_lexer_numbers() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"{"int": 42, "float": 3.14, "negative": -10, "scientific": 1e5}"#);

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let number_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == JsonSyntaxKind::Number).collect();

    assert_eq!(number_tokens.len(), 4);
}

#[test]
fn test_json_lexer_strings() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"{"simple": "hello", "escaped": "hello\nworld", "unicode": "\u0041"}"#);

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let string_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == JsonSyntaxKind::String).collect();

    assert_eq!(string_tokens.len(), 6); // 3 keys + 3 values
}

#[test]
fn test_json_lexer_whitespace() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new("  {\n  \"key\"  :  \"value\"\n}  ");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let whitespace_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == JsonSyntaxKind::Whitespace).collect();

    assert!(!whitespace_tokens.is_empty());
}

#[test]
fn test_json_lexer_error_handling() {
    let config = JsonLanguage::default();
    let lexer = JsonLexer::new(&config);
    let source = SourceText::new(r#"{"unterminated_string": "hello"#);

    let result = lexer.tokenize_source(&source);
    // Should have diagnostics for unterminated string
    assert!(!result.diagnostics.is_empty());
}
