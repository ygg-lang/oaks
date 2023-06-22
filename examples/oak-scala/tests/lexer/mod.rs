use oak_scala::{ScalaLanguage, lexer::ScalaLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_scala_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = ScalaLanguage::default();
    let lexer = ScalaLexer::new(&language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("scala").with_extension("dotty").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
