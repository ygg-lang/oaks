use oak_cmd::{builder::CmdBuilder, language::CmdLanguage};
use oak_testing::building::BuilderTester;
use std::time::Duration;

#[test]
fn test_builder() {
    let tester = BuilderTester::new(r"e:\普遍优化\oaks\examples\oak-cmd\tests\fixtures").with_extension(".cmd").with_timeout(Duration::from_millis(1000));

    let language = CmdLanguage::default();
    let builder = CmdBuilder::new(&language);
    tester.run_tests::<CmdLanguage, CmdBuilder>(&builder).unwrap();
}
