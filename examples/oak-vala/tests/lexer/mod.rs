use oak_vala::{ValaLanguage, ValaLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_vala_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ValaLexer::new(&ValaLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("vala");
    match test_runner.run_tests::<ValaLanguage, _>(lexer) {
        Ok(()) => println!("Vala lexer tests passed!"),
        Err(e) => panic!("Vala lexer tests failed: {}", e),
    }
}