use oak_testing::lexing::LexerTester;
use oak_twig::{language::TwigLanguage, lexer::TwigLexer};
use std::time::Duration;

#[test]
fn test_twig_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = TwigLanguage::default();
    let lexer = TwigLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("twig").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
