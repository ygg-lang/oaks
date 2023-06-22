use oak_core::{LexerState, helpers::LexerTester, source::Source};
use oak_wolfram::{WolframLanguage, WolframLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_wolfram_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(WolframLanguage::default()));
    let lexer = WolframLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("wl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<WolframLanguage, _>(&lexer) {
        Ok(()) => println!("Wolfram lexer tests passed!"),
        Err(e) => panic!("Wolfram lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::SourceText;
    use oak_wolfram::WolframLanguage;

    let source = SourceText::new("Module[{x}, x + 1]");
    let mut state = LexerState::<SourceText, WolframLanguage>::new(&source);

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
fn test_wolfram_function_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_wolfram::{WolframLanguage, WolframLexer};

    let source = SourceText::new("Module[{x}, x + 1]");
    let language = Box::leak(Box::new(WolframLanguage::default()));
    let lexer = WolframLexer::new(language);

    let mut cache = oak_core::ParseSession::<WolframLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("测试 Module[{{x}}, x + 1] 解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}', 位置={}..{}", first_token.kind, token_text, first_token.span.start, first_token.span.end);

    assert_eq!(token_text, "Module", "标识符应该被完整解析为 Module");
    assert_eq!(first_token.span.start, 0, "标记应该从位置 0 开始");
    assert_eq!(first_token.span.end, 6, "标记应该在位置 6 结束");

    println!("✅ Module 解析测试通过！");
}
