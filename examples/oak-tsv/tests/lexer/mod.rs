use oak_testing::lexing::LexerTester;
use oak_tsv::TsvLexer;
use std::{path::Path, time::Duration};

#[test]
fn test_tsv_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lexer = TsvLexer::new();
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("tsv").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
