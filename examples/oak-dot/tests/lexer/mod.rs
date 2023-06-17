use oak_core::helpers::LexerTester;
use oak_dot::{DotLanguage, DotLexer};
use std::path::Path;

#[test]
fn test_dot_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DotLexer::new(&DotLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("dot");
    match test_runner.run_tests::<DotLanguage, _>(lexer) {
        Ok(()) => println!("DOT lexer tests passed!"),
        Err(e) => panic!("DOT lexer tests failed: {}", e),
    }
}