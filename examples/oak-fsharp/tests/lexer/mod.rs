use oak_core::{LexerState, source::Source};
use oak_testing::lexing::LexerTester;
use oak_fsharp::{FSharpLanguage, FSharpLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_fsharp_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = FSharpLanguage::default();
    let lexer = FSharpLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("fs").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};
    use oak_fsharp::FSharpLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<&SourceText, FSharpLanguage>::new(&source);

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
fn test_fsharp_function_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_fsharp::{FSharpLanguage, FSharpLexer};

    let source = SourceText::new("let add x y = x + y\nlet result = add 1 2");
    let language = FSharpLanguage::default();
    let lexer = FSharpLexer::new(&language);

    let result = lexer.lex(&source);

    println!("测试 F# 函数解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}'", first_token.kind, token_text);

    println!("✅ F# 函数解析测试通过！");
}