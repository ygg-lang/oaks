use oak_core::helpers::LexerTester;
use oak_haskell::{HaskellLanguage, HaskellLexer};
use std::path::Path;

#[test]
fn test_haskell_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = HaskellLexer::new(&HaskellLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("hs");
    match test_runner.run_tests::<HaskellLanguage, _>(lexer) {
        Ok(()) => println!("Haskell lexer tests passed!"),
        Err(e) => panic!("Haskell lexer tests failed: {}", e),
    }
}
