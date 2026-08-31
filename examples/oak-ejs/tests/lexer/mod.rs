use oak_ejs::{language::EjsLanguage, lexer::EjsLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_ejs_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = EjsLanguage::default();
    let lexer = EjsLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("js").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
