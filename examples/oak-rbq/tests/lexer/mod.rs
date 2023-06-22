use oak_rbq::{RbqLanguage, RbqLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_rbq_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = RbqLanguage::default();
    let lexer = RbqLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("rbq").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
