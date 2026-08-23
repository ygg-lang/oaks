use oak_core::{errors::OakError, source::Source};
use oak_elixir::{ElixirLanguage, ElixirLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_elixir_lexer() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = ElixirLanguage::default();
    let lexer = ElixirLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests::<ElixirLanguage, _>(&lexer)?;
    Ok(())
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};
    use oak_elixir::ElixirLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut cache = oak_core::parser::session::ParseSession::<ElixirLanguage>::default();
    let mut state = LexerState::<SourceText, ElixirLanguage>::new_with_cache(&source, 0, &mut cache);

    println!("Initial state:");
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\nAfter advancing 1 char:");
    state.advance(1);
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\nAfter advancing 1 char:");
    state.advance(1);
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek())
}

#[test]
fn test_elixir_module_parsing() -> Result<(), OakError> {
    use oak_core::{Lexer, SourceText};

    let source = SourceText::new("defmodule MyModule do\n  def hello do\n    :world\n  end\nend");
    let language = ElixirLanguage::default();
    let lexer = ElixirLexer::new(&language);

    let mut cache = oak_core::parser::session::ParseSession::<ElixirLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("Testing Elixir module parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    let tokens = result.result?;
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}'", first_token.kind, token_text);

    println!("✅ Elixir module parsing test passed!");
    Ok(())
}
