use oak_core::helpers::LexerTester;
use oak_php::{language::PhpLanguage, lexer::PhpLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_php_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(PhpLanguage::default()));
    let lexer = PhpLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("php").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PhpLanguage, _>(&lexer) {
        Ok(()) => println!("PHP lexer tests passed!"),
        Err(e) => panic!("PHP lexer tests failed: {}", e),
    }
}
