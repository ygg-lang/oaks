use oak_cmd::{language::CmdLanguage, lexer::CmdLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_lexer() {
    let tester = LexerTester::new(r"e:\普遍优化\oaks\examples\oak-cmd\tests\fixtures").with_extension(".cmd").with_timeout(Duration::from_millis(1000));

    let language = CmdLanguage::default();
    let lexer = CmdLexer::new(&language);
    tester.run_tests::<CmdLanguage, CmdLexer>(&lexer).unwrap();
}
