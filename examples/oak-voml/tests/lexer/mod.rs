use oak_testing::lexing::LexerTester;
use oak_voml::{language::VLangLanguage as VomlLanguage, lexer::VLangLexer as VomlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_voml_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = VomlLanguage::default();
    let lexer = VomlLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("voml").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
