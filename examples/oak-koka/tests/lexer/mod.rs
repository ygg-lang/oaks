use oak_koka::{KokaLanguage, KokaLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_koka_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = KokaLanguage::default();
    let lexer = KokaLexer::new(&language);
    let test_runner = LexerTester::new(tests).with_extension("kk").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
