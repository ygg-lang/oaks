use oak_powershell::{PowerShellLanguage, PowerShellLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_powershell_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = PowerShellLexer::new(&PowerShellLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("ps1");
    match test_runner.run_tests::<PowerShellLanguage, _>(lexer) {
        Ok(()) => println!("PowerShell lexer tests passed!"),
        Err(e) => panic!("PowerShell lexer tests failed: {}", e),
    }
}