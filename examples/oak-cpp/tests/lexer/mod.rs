use oak_cpp::{CppLanguage, CppLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_cpp_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = CppLanguage::default();
    let lexer = CppLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("cpp").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}