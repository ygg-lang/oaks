use oak_core::helpers::LexerTester;
use oak_json::{language::JsonLanguage, lexer::JsonLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_json_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(JsonLanguage::standard()));
    let lexer = JsonLexer::new(language);
    // Use `txt` instead of `json` to avoid nested tests
    let test_runner = LexerTester::new(tests).with_extension("txt").with_timeout(Duration::from_secs(30));
    match test_runner.run_tests::<JsonLanguage, _>(lexer) {
        Ok(()) => println!("JSON lexer tests passed!"),
        Err(e) => panic!("JSON lexer tests failed: {}", e),
    }
}
