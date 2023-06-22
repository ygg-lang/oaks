use oak_tailwind::{language::TailwindLanguage, lexer::TailwindLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_tailwind_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = TailwindLanguage::default();
    let lexer = TailwindLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("tw").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
