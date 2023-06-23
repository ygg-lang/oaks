use oak_c::{CLanguage, CParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_c_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = CLanguage::default();
    let parser = CParser::new(&language);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("c").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
