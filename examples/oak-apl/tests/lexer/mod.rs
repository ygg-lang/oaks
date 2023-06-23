use oak_apl::{language::AplLanguage, lexer::AplLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_apl_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = AplLanguage::default();
    let lexer = AplLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("apl").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
