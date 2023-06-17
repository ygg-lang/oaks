use oak_core::helpers::LexerTester;
use oak_idl::{IdlLanguage, IdlLexer};
use std::path::Path;

#[test]
fn test_idl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = IdlLexer::new(&IdlLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("idl");
    match test_runner.run_tests::<IdlLanguage, _>(lexer) {
        Ok(()) => println!("IDL lexer tests passed!"),
        Err(e) => panic!("IDL lexer tests failed: {}", e),
    }
}