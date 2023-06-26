use oak_cmd::{language::CmdLanguage, parser::CmdParser};
use oak_testing::parsing::ParserTester;
use std::time::Duration;

#[test]
fn test_parser() {
    let tester = ParserTester::new(r"e:\普遍优化\oaks\examples\oak-cmd\tests\fixtures").with_extension(".cmd").with_timeout(Duration::from_millis(1000));

    let language = CmdLanguage::default();
    let parser = CmdParser::new(&language);
    tester.run_tests::<CmdLanguage, CmdParser>(&parser).unwrap();
}
