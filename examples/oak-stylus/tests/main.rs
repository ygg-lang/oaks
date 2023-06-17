use oak_core::{Lexer, Parser, SourceText};
use oak_toml::{TomlLanguage, TomlSyntaxKind};

#[test]
fn test_lexer_basic() {
    let language = TomlLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new(r#"key = "value""#);

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len());
}

#[test]
fn test_parser_basic() {
    let language = TomlLanguage::new();
    let parser = language.parser();
    let source = SourceText::new(r#"key = "value""#);

    let result = parser.parse(&source);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    println!("Parsed tree with {} children", tree.children.len());
}

#[test]
fn test_lexer_string() {
    let language = TomlLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new(r#""hello world""#);

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含字符串 kind
    let has_string = tokens.iter().any(|t| matches!(t.kind, TomlSyntaxKind::String));
    assert!(has_string, "Should contain a string token");
}

#[test]
fn test_lexer_number() {
    let language = TomlLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("123");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含数token
    let has_number = tokens.iter().any(|t| matches!(t.kind, TomlSyntaxKind::Integer | TomlSyntaxKind::Float));
    assert!(has_number, "Should contain a number token");
}

#[test]
fn test_lexer_boolean() {
    let language = TomlLanguage::new();
    let lexer = language.lexer();
    let source = SourceText::new("true");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含布尔token
    let has_boolean = tokens.iter().any(|t| matches!(t.kind, TomlSyntaxKind::Boolean));
    assert!(has_boolean, "Should contain a boolean token");
}

#[test]
fn test_parser_key_value() {
    let language = TomlLanguage::new();
    let parser = language.parser();
    let source = SourceText::new(r#"name = "John""#);

    let result = parser.parse(&source);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children");
}

#[test]
fn test_parser_table() {
    let language = TomlLanguage::new();
    let parser = language.parser();
    let source = SourceText::new(
        r#"[section]
name = "value"
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children");
}

#[test]
fn test_empty_input() {
    let language = TomlLanguage::new();
    let lexer = language.lexer();
    let parser = language.parser();
    let source = SourceText::new("");

    // 测试空输入的词法分析
    let lex_result = lexer.lex(&source);
    assert!(lex_result.result.is_ok());

    // 测试空输入的语法分析
    let parse_result = parser.parse(&source);
    assert!(parse_result.result.is_ok());
}
