use oak_markdown::{MarkdownLanguage, MarkdownLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_markdown_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = MarkdownLanguage::default();
    let lexer = MarkdownLexer::new(&language);
    let test_runner = LexerTester::new(tests).with_extension("markdown").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
