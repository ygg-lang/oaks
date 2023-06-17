use oak_wolfram::{WolframLanguage, WolframLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_wolfram_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = WolframLexer::new(&WolframLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("wl");
    match test_runner.run_tests::<WolframLanguage, _>(lexer) {
        Ok(()) => println!("Wolfram lexer tests passed!"),
        Err(e) => panic!("Wolfram lexer tests failed: {}", e),
    }
}