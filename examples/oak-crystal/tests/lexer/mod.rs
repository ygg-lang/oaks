use oak_core::helpers::LexerTester;
use oak_crystal::{CrystalLanguage, CrystalLexer};
use std::path::Path;

#[test]
fn test_crystal_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CrystalLexer::new(&CrystalLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("cr");
    match test_runner.run_tests::<CrystalLanguage, _>(lexer) {
        Ok(()) => println!("Crystal lexer tests passed!"),
        Err(e) => panic!("Crystal lexer tests failed: {}", e),
    }
}