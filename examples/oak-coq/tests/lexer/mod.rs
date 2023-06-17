use oak_core::helpers::LexerTester;
use oak_coq::{CppLanguage, CppLexer};
use std::path::Path;

#[test]
fn test_coq_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CppLexer::new(&CppLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("v");
    match test_runner.run_tests::<CppLanguage, _>(lexer) {
        Ok(()) => println!("Coq lexer tests passed!"),
        Err(e) => panic!("Coq lexer tests failed: {}", e),
    }
}