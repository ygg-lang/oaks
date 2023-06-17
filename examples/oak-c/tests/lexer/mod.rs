use oak_core::helpers::LexerTester;
use oak_c::{CLanguage, CLexer};
use std::path::Path;

#[test]
fn test_c_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CLexer::new(&CLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("c");
    match test_runner.run_tests::<CLanguage, _>(lexer) {
        Ok(()) => println!("C lexer tests passed!"),
        Err(e) => panic!("C lexer tests failed: {}", e),
    }
}
