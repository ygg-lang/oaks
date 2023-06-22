use oak_core::{Lexer, SourceText, helpers::LexerTester, source::Source};
use oak_regex::{RegexLanguage, RegexLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_regex_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RegexLanguage::default()));
    let lexer = RegexLexer::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("regex").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RegexLanguage, _>(&lexer) {
        Ok(()) => println!("Regex lexer tests passed!"),
        Err(e) => panic!("Regex lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};

    let source = SourceText::new(r"[a-z]+");
    let mut state = LexerState::<SourceText, RegexLanguage>::new(&source);

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
fn test_character_class_parsing() {
    let source = SourceText::new(r"[a-z]+");
    let language = Box::leak(Box::new(RegexLanguage::default()));
    let lexer = RegexLexer::new(language);

    let mut cache = oak_core::ParseSession::<RegexLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("测试字符类解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}', 位置={}..{}", first_token.kind, token_text, first_token.span.start, first_token.span.end);

    assert_eq!(token_text, "[", "第一个标记应该是左方括号");
    assert_eq!(first_token.span.start, 0, "标记应该从位置 0 开始");
    assert_eq!(first_token.span.end, 1, "标记应该在位置 1 结束");

    println!("✅ 字符类解析测试通过！");
}
