use oak_core::source::Source;
use oak_racket::{language::RacketLanguage, lexer::RacketLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_racket_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = RacketLanguage::default();
    let lexer = RacketLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("rkt").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
