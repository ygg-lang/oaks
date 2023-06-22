use oak_testing::lexing::LexerTester;
use oak_wit::{WitLanguage, WitLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_wit_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = WitLanguage::default();
    let lexer = WitLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("wit").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
