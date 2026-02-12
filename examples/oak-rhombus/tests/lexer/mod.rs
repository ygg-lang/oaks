use oak_core::source::Source;
use oak_rhombus::{language::RhombusLanguage, lexer::RhombusLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_rhombus_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = RhombusLanguage::default();
    let lexer = RhombusLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("rhm").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
