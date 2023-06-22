use oak_tcl::{TclLanguage, TclLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_tcl_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = TclLanguage::standard();
    let lexer = TclLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("tcl").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
