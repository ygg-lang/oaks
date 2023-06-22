use oak_perl::{language::PerlLanguage, lexer::PerlLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_perl_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = PerlLanguage::default();
    let lexer = PerlLexer::new(&language);

    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("pl").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
