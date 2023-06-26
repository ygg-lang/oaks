use oak_bat::{language::BatLanguage, parser::BatParser};
use oak_testing::parsing::ParserTester;
use std::time::Duration;

#[test]
fn test_parser() {
    let tester = ParserTester::new(r"e:\普遍优化\oaks\examples\oak-bat\tests\fixtures").with_extension(".bat").with_timeout(Duration::from_millis(1000));

    let language = BatLanguage::default();
    let parser = BatParser::new(&language);
    tester.run_tests::<BatLanguage, BatParser>(&parser).unwrap();
}
