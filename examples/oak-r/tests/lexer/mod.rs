use oak_core::helpers::LexerTester;
use oak_r::{language::RLanguage, lexer::RLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_r_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    static LANGUAGE: RLanguage = RLanguage;
    let lexer = RLexer::new(&LANGUAGE);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("r").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RLanguage, _>(&lexer) {
        Ok(()) => println!("R lexer tests passed!"),
        Err(e) => panic!("R lexer tests failed: {}", e),
    }
}
