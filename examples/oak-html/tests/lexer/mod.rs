use oak_core::helpers::LexerTester;
use oak_html::{HtmlLanguage, HtmlLexer};
use std::path::Path;

#[test]
fn test_html_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = HtmlLexer::new(&HtmlLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("html");
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML lexer tests passed!"),
        Err(e) => panic!("HTML lexer tests failed: {}", e),
    }
}
