use oak_core::{helpers::LexerTester, source::Source};
use oak_go::{GoLangLanguage, GoLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_go_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(GoLangLanguage::default()));
    let lexer = GoLexer::new(language);
    let test_runner =
        LexerTester::new(here.join("tests/lexer/fixtures")).with_extension("go").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<GoLangLanguage, _>(lexer) {
        Ok(()) => println!("Go lexer tests passed!"),
        Err(e) => panic!("Go lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{SourceText, lexer::LexerState};
    use oak_go::GoLangLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<&SourceText, GoLangLanguage>::new(&source);

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
fn test_go_function_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_go::{GoLangLanguage, GoLexer};

    let source = SourceText::new("package main");
    let language = Box::leak(Box::new(GoLangLanguage::default()));
    let lexer = GoLexer::new(language);

    let result = lexer.lex(&source);

    println!("测试 Go 函数解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}'", first_token.kind, token_text);

    println!("✅ Go 函数解析测试通过！");
}
