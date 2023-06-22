use oak_testing::lexing::LexerTester;
use oak_vlang::{VLangLanguage, VLangLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_vlang_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = VLangLanguage::default();
    let lexer = VLangLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("v").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}