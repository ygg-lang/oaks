use oak_core::{Lexer, helpers::LexerTester};
use oak_erlang::{ErlangLanguage, ErlangLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_erlang_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ErlangLanguage::default()));
    let lexer = ErlangLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("erl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ErlangLanguage, _>(lexer) {
        Ok(()) => println!("Erlang lexer tests passed!"),
        Err(e) => panic!("Erlang lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{SourceText, lexer::LexerState};

    let source = SourceText::new("hello world");
    let mut state = LexerState::<_, ErlangLanguage>::new(&source);

    println!("初始状态:");
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
fn test_erlang_module_parsing() {
    use oak_core::{SourceText, lexer::LexerState};
    use oak_erlang::{ErlangLanguage, ErlangLexer, ErlangSyntaxKind};

    let source = SourceText::new(
        r#"
-module(test).
-export([hello/0]).

hello() ->
    "Hello, World!".
"#,
    );

    let language = Box::leak(Box::new(ErlangLanguage::default()));
    let lexer = ErlangLexer::new(language);
    let result = lexer.lex(&source);

    // 验证包含模块声明的 token
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());

    println!("Parsed {} tokens:", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} at {:?}", i, token.kind, token.span);
    }

    // 检查是否有 EOF token
    assert_eq!(tokens.last().unwrap().kind, ErlangSyntaxKind::Eof);

    // 验证包含预期的 token 类型
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&ErlangSyntaxKind::Minus));
    assert!(token_kinds.contains(&ErlangSyntaxKind::Identifier));
}
