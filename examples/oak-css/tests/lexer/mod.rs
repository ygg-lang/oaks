use oak_core::{helpers::LexerTester, source::Source};
use oak_css::{CssLanguage, CssLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_css_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CssLanguage::default()));
    let lexer = CssLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("css").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CssLanguage, _>(&lexer) {
        Ok(()) => println!("CSS lexer tests passed!"),
        Err(e) => panic!("CSS lexer tests failed: {}", e),
    }
}