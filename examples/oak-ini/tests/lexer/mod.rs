use oak_core::helpers::LexerTester;
use oak_ini::{IniLanguage, IniLexer};
use std::path::Path;

#[test]
fn test_ini_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = IniLexer::new(&IniLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("ini");
    match test_runner.run_tests::<IniLanguage, _>(lexer) {
        Ok(()) => println!("INI lexer tests passed!"),
        Err(e) => panic!("INI lexer tests failed: {}", e),
    }
}