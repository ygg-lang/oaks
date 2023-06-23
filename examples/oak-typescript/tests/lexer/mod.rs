use oak_testing::lexing::LexerTester;
use oak_typescript::{language::TypeScriptLanguage, lexer::TypeScriptLexer};
use std::{path::Path, time::Duration};

lazy_static::lazy_static! {
    static ref LANGUAGE: TypeScriptLanguage = TypeScriptLanguage::standard();
}

#[test]
fn test_typescript_lexer() -> Result<(), oak_core::OakError> {
    let lexer = TypeScriptLexer::new(&LANGUAGE);
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lexer");
    let tester = LexerTester::new(test_dir).with_extension("ts").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
