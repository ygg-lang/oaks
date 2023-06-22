use oak_testing::lexing::LexerTester;
use oak_voc::{language::VocLanguage, lexer::VocLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_voc_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = VocLanguage::default();
    let lexer = VocLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("voc").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
