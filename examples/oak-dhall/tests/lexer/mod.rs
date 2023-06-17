use oak_core::helpers::LexerTester;
use oak_dhall::{DhallLanguage, DhallLexer};
use std::path::Path;

#[test]
fn test_dhall_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DhallLexer::new(&DhallLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("dhall");
    match test_runner.run_tests::<DhallLanguage, _>(lexer) {
        Ok(()) => println!("Dhall lexer tests passed!"),
        Err(e) => panic!("Dhall lexer tests failed: {}", e),
    }
}