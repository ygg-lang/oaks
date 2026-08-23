use oak_core::{Lexer, ParseSession, Parser, SourceText};
use oak_tailwind::{TailwindLanguage, TailwindTokenType, TailwindLexer, TailwindParser};

mod lexer;

#[test]
fn test_lexer_basic() {
    let language = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&language);
    let source = SourceText::new("{{ variable }}");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len())
}

#[test]
fn test_parser_basic() {
    let language = TailwindLanguage::default();
    let parser = TailwindParser::new(&language);
    let source = SourceText::new("{{ variable }}");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    println!("Parsed tree with {} children", tree.children.len())
}

#[test]
fn test_lexer_string() {
    let language = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&language);
    let source = SourceText::new(r#"{{ "hello world" }}"#);
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含字符串 kind
    let has_string = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::String));
    assert!(has_string, "Should contain a string token")
}

#[test]
fn test_lexer_number() {
    let language = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&language);
    let source = SourceText::new("{{ 123 }}");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含数字token
    let has_number = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::Number));
    assert!(has_number, "Should contain a number token")
}

#[test]
fn test_lexer_boolean() {
    let language = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&language);
    let source = SourceText::new("{{ true }}");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = lexer.lex(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否包含布尔token
    let has_boolean = tokens.iter().any(|t| matches!(t.kind, TailwindTokenType::Boolean));
    assert!(has_boolean, "Should contain a boolean token")
}

#[test]
fn test_parser_variable() {
    let language = TailwindLanguage::default();
    let parser = TailwindParser::new(&language);
    let source = SourceText::new("{{ name }}");
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_parser_block() {
    let language = TailwindLanguage::default();
    let parser = TailwindParser::new(&language);
    let source = SourceText::new(
        r#"{% if condition %}
    Hello World
{% endif %}"#,
    );
    let mut session = ParseSession::<TailwindLanguage>::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.result.is_ok());

    let tree = result.result.unwrap();
    assert!(!tree.children.is_empty(), "Parsed tree should have children")
}

#[test]
fn test_empty_input() {
    let language = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&language);
    let parser = TailwindParser::new(&language);
    let source = SourceText::new("");
    let mut session = ParseSession::<TailwindLanguage>::default();

    // 测试空输入的词法分析
    let lex_result = lexer.lex(&source, &[], &mut session);
    assert!(lex_result.result.is_ok());

    // 测试空输入的语法分析
    let parse_result = parser.parse(&source, &[], &mut session);
    assert!(parse_result.result.is_ok())
}
