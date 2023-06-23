use oak_r::{language::RLanguage, lexer::RLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_r_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RLanguage {}));
    let lexer = RLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("r").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RLanguage, _>(&lexer) {
        Ok(()) => println!("R lexer tests passed!"),
        Err(e) => panic!("R lexer tests failed: {}", e),
    }
}
