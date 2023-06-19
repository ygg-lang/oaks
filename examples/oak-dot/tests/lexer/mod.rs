use oak_core::helpers::LexerTester;
use oak_dot::{DotLanguage, DotLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_dot_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(DotLanguage::default()));
    let lexer = DotLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("dot").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DotLanguage, _>(lexer) {
        Ok(()) => println!("DOT lexer tests passed!"),
        Err(e) => panic!("DOT lexer tests failed: {}", e),
    }
}
