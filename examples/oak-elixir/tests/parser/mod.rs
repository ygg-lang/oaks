use oak_core::errors::OakError;
use oak_elixir::{ElixirLanguage, ElixirParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_elixir_parser() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer"); // Use lexer tests as basic examples
    let language = ElixirLanguage::default();
    let parser = ElixirParser::new(language);
    let test_runner = ParserTester::new(tests).with_extension("ex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)?;
    Ok(())
}
