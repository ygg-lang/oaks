use oak_regex::{RegexLanguage, RegexParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_regex_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lang = RegexLanguage::default();
    let parser = RegexParser::new(&lang);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("regex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
