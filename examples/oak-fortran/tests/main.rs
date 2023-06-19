mod lexer;

use oak_core::helpers::LexerTester;
use oak_fortran::{FortranLanguage, FortranLexer};
use std::path::PathBuf;

#[test]
fn test_fortran_lexer_integration() {
    let config = Box::leak(Box::new(FortranLanguage::default()));
    let lexer = FortranLexer::new(config);

    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests");
    test_dir.push("lexer");
    test_dir.push("fixtures");

    LexerTester::new(test_dir).with_extension("f90").run_tests(lexer).expect("Lexer tests should pass");
}
