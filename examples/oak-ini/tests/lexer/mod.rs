use oak_core::helpers::LexerTester;
use oak_ini::{IniLanguage, IniLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_ini_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(IniLanguage::default()));
    let lexer = IniLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ini").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<IniLanguage, _>(&lexer) {
        Ok(()) => println!("INI lexer tests passed!"),
        Err(e) => panic!("INI lexer tests failed: {}", e),
    }
}