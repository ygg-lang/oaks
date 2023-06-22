use oak_lean::{LeanLanguage, LeanLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_lean_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = LeanLanguage::default();
    let lexer = LeanLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("lean").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
