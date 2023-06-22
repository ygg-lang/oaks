use oak_html::{HtmlLanguage, HtmlLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_html_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = HtmlLanguage::default();
    let lexer = HtmlLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
