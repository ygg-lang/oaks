use oak_core::Source;
use oak_rust::lexer::RustTokenType;

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};
    use oak_rust::RustLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<SourceText, RustLanguage>::new(&source);

    println!("初始状态:");
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());
}

#[test]
fn test_nested_constant_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let source = SourceText::new("NESTED_CONSTANT");
    let language = Box::leak(Box::new(RustLanguage::default()));
    let lexer = RustLexer::new(language);

    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("测试 NESTED_CONSTANT 解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}', 位置={}..{}", first_token.kind, token_text, first_token.span.start, first_token.span.end);

    // 验证标识符类型
    assert!(matches!(first_token.kind, RustTokenType::Identifier), "应该是标识符类型");
    assert_eq!(token_text, "NESTED_CONSTANT", "标识符应该被完整解析为 NESTED_CONSTANT");
    assert_eq!(first_token.span.start, 0, "标记应该从位置 0 开始");
    assert_eq!(first_token.span.end, 15, "标记应该在位置 15 结束");

    println!("✅ NESTED_CONSTANT 解析测试通过！");
}

#[test]
fn test_rust_lexer_keywords() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new("fn let mut pub struct enum trait impl use");
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    // Should have tokens for each keyword plus whitespace
    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(tokens.len() >= 8, "Should tokenize all keywords");

    println!("Rust lexer keywords test passed - {} tokens generated", tokens.len());
}

#[test]
fn test_rust_lexer_literals() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new(r#"42 3.14 "hello" 'a' true false"#);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty(), "Should tokenize literals");

    println!("Rust lexer literals test passed - {} tokens generated", tokens.len());
}

#[test]
fn test_rust_lexer_comments() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new(
        r#"
// Line comment
/* Block comment */
/// Doc comment
//! Module doc
"#,
    );
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty(), "Should tokenize comments");

    println!("Rust lexer comments test passed - {} tokens generated", tokens.len());
}
