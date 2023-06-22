use oak_vhdl::{VhdlLanguage, VhdlLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_vhdl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = VhdlLexer::new(&VhdlLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("vhd");
    match test_runner.run_tests::<VhdlLanguage, _>(&lexer) {
        Ok(()) => println!("VHDL lexer tests passed!"),
        Err(e) => panic!("VHDL lexer tests failed: {}", e),
    }
}