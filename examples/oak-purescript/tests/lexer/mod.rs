use oak_core::helpers::LexerTester;
use oak_purescript::{language::PurescriptLanguage, lexer::PurescriptLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_purescript_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(PurescriptLanguage::default()));
    let lexer = PurescriptLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("purs").with_timeout(Duration::from_secs(5));

    match test_runner.run_tests::<PurescriptLanguage, _>(lexer) {
        Ok(_) => println!("All PureScript lexer tests passed!"),
        Err(e) => panic!("PureScript lexer tests failed: {}", e),
    }
}
