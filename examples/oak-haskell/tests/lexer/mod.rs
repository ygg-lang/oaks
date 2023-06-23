use oak_haskell::{HaskellLanguage, HaskellLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_haskell_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HaskellLanguage::default()));
    let lexer = HaskellLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("hs").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HaskellLanguage, _>(&lexer) {
        Ok(()) => println!("Haskell lexer tests passed!"),
        Err(e) => panic!("Haskell lexer tests failed: {}", e),
    }
}
