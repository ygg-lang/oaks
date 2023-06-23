use oak_testing::lexing::LexerTester;
use oak_wgsl::{WgslLanguage, WgslLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_wgsl_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(WgslLanguage::default()));
    let lexer = WgslLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("wgsl").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
