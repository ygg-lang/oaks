use oak_core::helpers::LexerTester;
use oak_cobol::{CobolLanguage, CobolLexer};
use std::path::Path;

#[test]
fn test_cobol_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CobolLexer::new(&CobolLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("cobol");
    match test_runner.run_tests::<CobolLanguage, _>(lexer) {
        Ok(()) => println!("COBOL lexer tests passed!"),
        Err(e) => panic!("COBOL lexer tests failed: {}", e),
    }
}