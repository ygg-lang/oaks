use oak_testing::lexing::LexerTester;
use oak_typescript::{language::TypeScriptLanguage, lexer::TypeScriptLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_typescript_lexer() -> Result<(), oak_core::OakError> {
    let language = TypeScriptLanguage::standard();
    let lexer = TypeScriptLexer::new(&language);
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lexer");
    let tester = LexerTester::new(test_dir).with_extension("ts").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
