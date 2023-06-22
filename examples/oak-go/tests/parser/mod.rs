use oak_go::{GoLanguage, GoParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_go_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = GoLanguage::default();
    let parser = GoParser::new(language);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("go").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
