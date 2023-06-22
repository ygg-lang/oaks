use oak_javadoc::{language::JavadocLanguage, lexer::JavadocLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_javadoc_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = JavadocLanguage::default();
    let lexer = JavadocLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("javadoc").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
