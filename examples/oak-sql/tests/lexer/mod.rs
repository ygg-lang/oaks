use oak_sql::{SqlLanguage, SqlLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_sql_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = SqlLanguage::default();
    let lexer = SqlLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("sql").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
