use oak_solidity::{SolidityLanguage, SolidityLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_solidity_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = SolidityLanguage::default();
    let lexer = SolidityLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("sol").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
