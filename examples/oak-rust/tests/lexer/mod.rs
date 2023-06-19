#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_rust::{RustLanguage, RustLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_rust_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RustLanguage::default()));
    let lexer = RustLexer::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("rust").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RustLanguage, _>(lexer) {
        Ok(()) => println!("Rust lexer tests passed!"),
        Err(e) => panic!("Rust lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{SourceText, lexer::LexerState};
    use oak_rust::RustLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<&SourceText, RustLanguage>::new(&source);

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

    let result = lexer.lex(&source);

    println!("测试 NESTED_CONSTANT 解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!(
        "第一个标记: 类型={:?}, 文本='{}', 位置={}..{}",
        first_token.kind, token_text, first_token.span.start, first_token.span.end
    );

    assert_eq!(token_text, "NESTED_CONSTANT", "标识符应该被完整解析为 NESTED_CONSTANT");
    assert_eq!(first_token.span.start, 0, "标记应该从位置 0 开始");
    assert_eq!(first_token.span.end, 15, "标记应该在位置 15 结束");

    println!("✅ NESTED_CONSTANT 解析测试通过！");
}
