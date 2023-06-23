use oak_sass::{SassLanguage, lexer::SassLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
#[cfg(feature = "serde")]
fn test_sass_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(SassLanguage::default()));
    let lexer = SassLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("sass").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
