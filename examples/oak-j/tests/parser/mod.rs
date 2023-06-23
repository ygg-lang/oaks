use oak_j::{language::JLanguage, parser::JParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_J_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = JLanguage::default();
    let parser = JParser::new(&language);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("J").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
