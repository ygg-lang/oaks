use oak_core::helpers::LexerTester;
use oak_fortran::{FortranLanguage, FortranLexer};
use std::path::Path;

#[test]
fn test_fortran_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = FortranLexer::new(&FortranLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("f90");
    match test_runner.run_tests::<FortranLanguage, _>(lexer) {
        Ok(()) => println!("Fortran lexer tests passed!"),
        Err(e) => panic!("Fortran lexer tests failed: {}", e),
    }
}