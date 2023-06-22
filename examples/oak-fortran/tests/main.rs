mod lexer;

use oak_fortran::{FortranLanguage, FortranLexer};
use oak_testing::lexing::LexerTester;
use std::path::PathBuf;

#[test]
fn test_fortran_lexer_integration() -> Result<(), oak_core::OakError> {
    let language = FortranLanguage::default();
    let lexer = FortranLexer::new(&language);

    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests");
    test_dir.push("lexer");
    test_dir.push("fixtures");

    LexerTester::new(test_dir).with_extension("f90").with_timeout(std::time::Duration::from_secs(5)).run_tests(&lexer)
}
