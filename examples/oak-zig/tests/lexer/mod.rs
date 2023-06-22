use oak_testing::lexing::LexerTester;
use oak_zig::{ZigLanguage, ZigLexer};
use std::time::Duration;

#[test]
fn test_zig_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = ZigLanguage::default();
    let lexer = ZigLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("zig").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
