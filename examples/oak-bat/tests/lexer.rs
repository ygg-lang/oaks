use oak_bat::{language::BatLanguage, lexer::BatLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_lexer() {
    let tester = LexerTester::new(r"e:\普遍优化\oaks\examples\oak-bat\tests\fixtures").with_extension(".bat").with_timeout(Duration::from_millis(1000));

    let language = BatLanguage::default();
    let lexer = BatLexer::new(&language);
    tester.run_tests::<BatLanguage, BatLexer>(&lexer).unwrap();
}
