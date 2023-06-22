use oak_core::{helpers::LexerTester, source::Source};
use oak_elixir::{ElixirLanguage, ElixirLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_elixir_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ElixirLanguage::default()));
    let lexer = ElixirLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ex").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ElixirLanguage, _>(&lexer) {
        Ok(()) => println!("Elixir lexer tests passed!"),
        Err(e) => panic!("Elixir lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};
    use oak_elixir::ElixirLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut cache = oak_core::parser::session::ParseSession::<ElixirLanguage>::default();
    let mut state = LexerState::<SourceText, ElixirLanguage>::new_with_cache(&source, 0, &mut cache);

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
fn test_elixir_module_parsing() {
    use oak_core::{Lexer, SourceText};

    let source = SourceText::new("defmodule MyModule do\n  def hello do\n    :world\n  end\nend");
    let language = Box::leak(Box::new(ElixirLanguage::default()));
    let lexer = ElixirLexer::new(language);

    let mut cache = oak_core::parser::session::ParseSession::<ElixirLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("测试 Elixir 模块解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}'", first_token.kind, token_text);

    println!("✅ Elixir 模块解析测试通过！");
}
