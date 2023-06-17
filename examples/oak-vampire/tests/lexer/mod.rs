use oak_vampire::{VampireLanguage, VampireLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_vampire_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = VampireLexer::new(&VampireLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("tptp");
    match test_runner.run_tests::<VampireLanguage, _>(lexer) {
        Ok(()) => println!("Vampire lexer tests passed!"),
        Err(e) => panic!("Vampire lexer tests failed: {}", e),
    }
}