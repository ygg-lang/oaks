use oak_core::helpers::LexerTester;
use oak_dart::{DartLanguage, DartLexer};
use std::path::Path;

#[test]
fn test_dart_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DartLexer::new(&DartLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("dart");
    match test_runner.run_tests::<DartLanguage, _>(lexer) {
        Ok(()) => println!("Dart lexer tests passed!"),
        Err(e) => panic!("Dart lexer tests failed: {}", e),
    }
}