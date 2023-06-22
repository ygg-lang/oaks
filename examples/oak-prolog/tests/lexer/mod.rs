use oak_core::helpers::LexerTester;
use oak_prolog::{language::PrologLanguage, lexer::PrologLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_prolog_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(PrologLanguage::default()));
    let lexer = PrologLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("pl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PrologLanguage, _>(&lexer) {
        Ok(()) => println!("Prolog lexer tests passed!"),
        Err(e) => panic!("Prolog lexer tests failed: {}", e),
    }
}
