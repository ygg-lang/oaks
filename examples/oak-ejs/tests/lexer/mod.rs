use oak_javascript::{language::JavaScriptLanguage, lexer::JavaScriptLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_javascript_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(JavaScriptLanguage::standard()));
    let lexer = JavaScriptLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("js").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
