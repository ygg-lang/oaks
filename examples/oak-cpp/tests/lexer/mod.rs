use oak_core::helpers::LexerTester;
use oak_cpp::{CppLanguage, CppLexer};
use std::path::Path;

#[test]
fn test_cpp_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CppLexer::new(&CppLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("cpp");
    match test_runner.run_tests::<CppLanguage, _>(lexer) {
        Ok(()) => println!("C++ lexer tests passed!"),
        Err(e) => panic!("C++ lexer tests failed: {}", e),
    }
}