use oak_testing::parsing::ParserTester;
use oak_zig::{ZigLanguage, ZigParser};
use std::time::Duration;

#[test]
fn test_zig_parser() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/parser");
    let config = ZigLanguage::default();
    let parser = ZigParser::new(config);
    let tester = ParserTester::new(tests).with_extension("zig").with_timeout(Duration::from_secs(5));
    tester.run_tests(&parser)
}
