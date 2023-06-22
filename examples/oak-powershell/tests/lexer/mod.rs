use oak_powershell::{language::PowerShellLanguage, lexer::PowerShellLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_powershell_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = PowerShellLanguage::default();
    let lexer = PowerShellLexer::new(&language);
    let test_runner = LexerTester::new(tests).with_extension("ps1").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
