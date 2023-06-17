use oak_core::{SourceText, lexer::Lexer};
use oak_toml::{TomlLanguage, TomlLexer, TomlSyntaxKind};

#[test]
fn test_toml_lexer_basic_tokens() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"title = "TOML Example"
[owner]
name = "Tom Preston-Werner"
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check for expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&TomlSyntaxKind::Identifier));
    assert!(token_kinds.contains(&TomlSyntaxKind::Equals));
    assert!(token_kinds.contains(&TomlSyntaxKind::String));
    assert!(token_kinds.contains(&TomlSyntaxKind::LeftBracket));
    assert!(token_kinds.contains(&TomlSyntaxKind::RightBracket));
}

#[test]
fn test_toml_lexer_tables() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"[table1]
[table2.subtable]
[[array_of_tables]]
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let bracket_tokens: Vec<_> =
        tokens.iter().filter(|t| matches!(t.kind, TomlSyntaxKind::LeftBracket | TomlSyntaxKind::RightBracket)).collect();

    assert_eq!(bracket_tokens.len(), 8); // [table1], [table2.subtable], [[array_of_tables]]
}

#[test]
fn test_toml_lexer_key_value_pairs() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"string = "hello"
integer = 42
float = 3.14
boolean = true
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let equals_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::Equals).collect();

    assert_eq!(equals_tokens.len(), 4);
}

#[test]
fn test_toml_lexer_arrays() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"numbers = [1, 2, 3]
strings = ["a", "b", "c"]
mixed = [1, "two", 3.0, true]
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let bracket_tokens: Vec<_> =
        tokens.iter().filter(|t| matches!(t.kind, TomlSyntaxKind::LeftBracket | TomlSyntaxKind::RightBracket)).collect();

    let comma_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::Comma).collect();

    assert_eq!(bracket_tokens.len(), 6); // 3 arrays * 2 brackets each
    assert_eq!(comma_tokens.len(), 8); // Total commas in all arrays
}

#[test]
fn test_toml_lexer_inline_tables() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"point = { x = 1, y = 2 }
color = { r = 255, g = 128, b = 0 }
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let brace_tokens: Vec<_> =
        tokens.iter().filter(|t| matches!(t.kind, TomlSyntaxKind::LeftBrace | TomlSyntaxKind::RightBrace)).collect();

    assert_eq!(brace_tokens.len(), 4); // 2 inline tables * 2 braces each
}

#[test]
fn test_toml_lexer_comments() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"# This is a comment
key = "value" # End of line comment
# Another comment
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let comment_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::Comment).collect();

    assert_eq!(comment_tokens.len(), 3);
}

#[test]
fn test_toml_lexer_strings() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"basic = "I'm a string"
multiline = """
Here are two
lines"""
literal = 'C:\Users\nodejs\templates'
multiline_literal = '''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let string_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::String).collect();

    assert_eq!(string_tokens.len(), 4);
}

#[test]
fn test_toml_lexer_numbers() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"integer = 42
negative = -17
float = 3.14159
scientific = 5e+22
underscore = 1_000_000
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let number_tokens: Vec<_> =
        tokens.iter().filter(|t| matches!(t.kind, TomlSyntaxKind::Integer | TomlSyntaxKind::Float)).collect();

    assert_eq!(number_tokens.len(), 5);
}

#[test]
fn test_toml_lexer_booleans() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"enabled = true
disabled = false
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let bool_tokens: Vec<_> =
        tokens.iter().filter(|t| matches!(t.kind, TomlSyntaxKind::TrueKeyword | TomlSyntaxKind::FalseKeyword)).collect();

    assert_eq!(bool_tokens.len(), 2);
}

#[test]
fn test_toml_lexer_datetime() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"date1 = 1979-05-27T07:32:00Z
date2 = 1979-05-27T00:32:00-07:00
date3 = 1979-05-27T00:32:00.999999-07:00
date4 = 1979-05-27
time1 = 07:32:00
time2 = 00:32:00.999999
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let datetime_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::DateTime).collect();

    assert_eq!(datetime_tokens.len(), 6);
}

#[test]
fn test_toml_lexer_dots() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"[table.subtable]
key.subkey = "value"
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let dot_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TomlSyntaxKind::Dot).collect();

    assert_eq!(dot_tokens.len(), 2);
}

#[test]
fn test_toml_lexer_complex_example() {
    let config = TomlLanguage::default();
    let lexer = TomlLexer::new(&config);
    let source = SourceText::new(
        r#"# This is a TOML document

title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00

[database]
enabled = true
ports = [ 8000, 8001, 8002 ]
data = [ ["gamma", "delta"], [1, 2] ]

[servers]

[servers.alpha]
ip = "10.0.0.1"
role = "frontend"

[servers.beta]
ip = "10.0.0.2"
role = "backend"
"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Verify we have all expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&TomlSyntaxKind::Comment));
    assert!(token_kinds.contains(&TomlSyntaxKind::Identifier));
    assert!(token_kinds.contains(&TomlSyntaxKind::Equals));
    assert!(token_kinds.contains(&TomlSyntaxKind::String));
    assert!(token_kinds.contains(&TomlSyntaxKind::LeftBracket));
    assert!(token_kinds.contains(&TomlSyntaxKind::RightBracket));
    assert!(token_kinds.contains(&TomlSyntaxKind::DateTime));
    assert!(token_kinds.contains(&TomlSyntaxKind::TrueKeyword));
    assert!(token_kinds.contains(&TomlSyntaxKind::Integer));
    assert!(token_kinds.contains(&TomlSyntaxKind::Comma));
    assert!(token_kinds.contains(&TomlSyntaxKind::Dot));
}
