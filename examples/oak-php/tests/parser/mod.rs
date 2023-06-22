use oak_php::{language::PhpLanguage, parser::PhpParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_php_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = PhpLanguage::default();
    let parser = PhpParser::new(language);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("php").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
