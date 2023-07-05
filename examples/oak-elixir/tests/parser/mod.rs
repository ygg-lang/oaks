use oak_core::{
    errors::OakError,
    parser::{ParseSession, Parser},
};
use oak_elixir::{ElixirLanguage, parser::ElixirParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_elixir_parser_basic() {
    let lang = ElixirLanguage::default();
    let parser = ElixirParser::new(&lang);
    let source = "defmodule Hello do\n  def world do\n    IO.puts \"Hello\"\n  end\nend";
    let mut session = ParseSession::<ElixirLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.result.is_ok());
    let green = output.result.unwrap();
    assert!(!green.children.is_empty());
}

#[test]
fn test_elixir_parser_suite() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer"); // Use lexer tests as basic examples
    let language = ElixirLanguage::default();
    let parser = ElixirParser::new(&language);
    let test_runner = ParserTester::new(tests).with_extension("ex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)?;
    Ok(())
}
