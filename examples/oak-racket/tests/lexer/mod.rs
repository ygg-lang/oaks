use oak_racket::lexer::Lexer;
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_racket_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = Lexer;
    let tester = LexerTester::new(tests).with_extension("rkt").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
