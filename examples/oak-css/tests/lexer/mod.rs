use oak_core::helpers::LexerTester;
use oak_css::{CssLanguage, CssLexer};
use std::path::Path;

#[test]
fn test_css_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CssLexer::new(&CssLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("css");
    match test_runner.run_tests::<CssLanguage, _>(lexer) {
        Ok(()) => println!("CSS lexer tests passed!"),
        Err(e) => panic!("CSS lexer tests failed: {}", e),
    }
}