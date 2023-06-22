use oak_vlang::{VLangLanguage, VLangLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_vlang_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = VLangLexer::new(&VLangLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("v");
    match test_runner.run_tests::<VLangLanguage, _>(&lexer) {
        Ok(()) => println!("V language lexer tests passed!"),
        Err(e) => panic!("V language lexer tests failed: {}", e),
    }
}