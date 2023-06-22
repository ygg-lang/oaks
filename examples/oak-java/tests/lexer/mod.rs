use oak_java::{JavaLanguage, JavaLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_java_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = JavaLanguage::default();
    let lexer = JavaLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("java").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
