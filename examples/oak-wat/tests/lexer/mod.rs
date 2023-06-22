use oak_diagnostic::testing::lexing::LexerTester;
use oak_wat::{WatLanguage, WatLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_wat_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(WatLanguage::default()));
    let lexer = WatLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("wat").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<WatLanguage, _>(&lexer) {
        Ok(()) => println!("WAT lexer tests passed!"),
        Err(e) => panic!("WAT lexer tests failed: {}", e),
    }
}
