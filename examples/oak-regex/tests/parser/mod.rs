use oak_core::helpers::ParserTester;
use oak_regex::{RegexLanguage, RegexParser};
use std::{path::Path, time::Duration};

#[test]
fn test_regex_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lang: &'static RegexLanguage = Box::leak(Box::new(RegexLanguage::default()));
    let parser: &'static RegexParser = Box::leak(Box::new(RegexParser::new(lang)));
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("regex").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RegexLanguage, _>(parser) {
        Ok(()) => println!("Regex files tests passed!"),
        Err(e) => panic!("Regex files tests failed: {}", e),
    }
}
