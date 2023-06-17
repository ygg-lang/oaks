use oak_core::helpers::LexerTester;
use oak_python::{PythonLanguage, PythonLexer};
use std::path::Path;

#[test]
fn test_python_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = PythonLexer::new(&PythonLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("py");
    match test_runner.run_tests::<PythonLanguage, _>(lexer) {
        Ok(()) => println!("Python lexer tests passed!"),
        Err(e) => panic!("Python lexer tests failed: {}", e),
    }
}