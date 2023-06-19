use oak_core::helpers::LexerTester;
use oak_python::{PythonLanguage, PythonLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_python_lexer() {
    let language = Box::leak(Box::new(PythonLanguage::default()));
    let lexer = PythonLexer::new(language);
    let test_runner = LexerTester::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lexer"))
        .with_extension("py")
        .with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PythonLanguage, _>(lexer) {
        Ok(()) => println!("Python lexer tests passed!"),
        Err(e) => panic!("Python lexer tests failed: {}", e),
    }
}
