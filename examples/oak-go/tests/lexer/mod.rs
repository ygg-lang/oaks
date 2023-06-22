use oak_core::source::Source;
use oak_go::{GoLanguage, GoLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_go_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = GoLanguage::default();
    let lexer = GoLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer/fixtures")).with_extension("go").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
