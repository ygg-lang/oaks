use oak_core::helpers::LexerTester;
use oak_lean::{LeanLanguage, LeanLexer};
use std::path::Path;

#[test]
fn test_lean_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let binding = LeanLanguage::default();
    let lexer = LeanLexer::new(&binding);
    let test_runner = LexerTester::new(tests).with_extension("lean");
    match test_runner.run_tests::<LeanLanguage, _>(lexer) {
        Ok(()) => println!("Lean lexer tests passed!"),
        Err(e) => panic!("Lean lexer tests failed: {}", e),
    }
}
