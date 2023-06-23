use oak_python::{PythonLanguage, PythonLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_python_lexer() -> Result<(), oak_core::OakError> {
    let language = Box::leak(Box::new(PythonLanguage::default()));
    let lexer = PythonLexer::new(language);
    let test_runner = LexerTester::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lexer")).with_extension("py").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
