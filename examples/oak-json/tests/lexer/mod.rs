use oak_core::helpers::LexerTester;
use oak_json::{JsonLanguage, JsonLexer};
use std::path::Path;

#[test]
fn test_json_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = JsonLexer::new(&JsonLanguage::standard());
    /// Use `txt` instead of `json` to avoid nested tests
    let test_runner = LexerTester::new(tests).with_extension("txt");
    match test_runner.run_tests::<JsonLanguage, _>(lexer) {
        Ok(()) => println!("JSON lexer tests passed!"),
        Err(e) => panic!("JSON lexer tests failed: {}", e),
    }
}
