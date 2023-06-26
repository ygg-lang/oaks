use oak_bat::{builder::BatBuilder, language::BatLanguage};
use oak_testing::building::BuilderTester;
use std::time::Duration;

#[test]
fn test_builder() {
    let tester = BuilderTester::new(r"e:\普遍优化\oaks\examples\oak-bat\tests\fixtures").with_extension(".bat").with_timeout(Duration::from_millis(1000));

    let language = BatLanguage::default();
    let builder = BatBuilder::new(&language);
    tester.run_tests::<BatLanguage, BatBuilder>(&builder).unwrap();
}
