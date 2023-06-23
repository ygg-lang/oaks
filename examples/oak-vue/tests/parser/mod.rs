use oak_core::errors::OakError;
use oak_testing::parsing::ParserTester;
use oak_vue::{VueLanguage, VueParser};
use std::{path::Path, time::Duration};

#[test]
fn test_vue_parser() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/parser");
    let language = Box::leak(Box::new(VueLanguage::default()));
    let parser = VueParser::new(language);
    let test_runner = ParserTester::new(tests).with_extension("vue").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
