use oak_core::helpers::LexerTester;
use oak_erlang::{ErlangLanguage, ErlangLexer};
use std::path::Path;

#[test]
fn test_erlang_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ErlangLexer::new(&ErlangLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("erl");
    match test_runner.run_tests::<ErlangLanguage, _>(lexer) {
        Ok(()) => println!("Erlang lexer tests passed!"),
        Err(e) => panic!("Erlang lexer tests failed: {}", e),
    }
}