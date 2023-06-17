use oak_ada::{AdaLanguage, AdaLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_ada_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = AdaLexer::new(&AdaLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("ada");
    match test_runner.run_tests::<AdaLanguage, _>(lexer) {
        Ok(()) => println!("Ada lexer tests passed!"),
        Err(e) => panic!("Ada lexer tests failed: {}", e),
    }
}
