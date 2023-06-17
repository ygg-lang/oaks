use oak_core::helpers::LexerTester;
use oak_elixir::{ElixirLanguage, ElixirLexer};
use std::path::Path;

#[test]
fn test_elixir_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ElixirLexer::new(&ElixirLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("ex");
    match test_runner.run_tests::<ElixirLanguage, _>(lexer) {
        Ok(()) => println!("Elixir lexer tests passed!"),
        Err(e) => panic!("Elixir lexer tests failed: {}", e),
    }
}