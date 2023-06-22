mod lexer;

use oak_core::helpers::LexerTester;
use oak_powershell::{language::PowerShellLanguage, lexer::PowerShellLexer};
use std::{path::Path, time::Duration};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_powershell_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(PowerShellLanguage::default()));
    let lexer = PowerShellLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ps1").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PowerShellLanguage, _>(&lexer) {
        Ok(()) => println!("PowerShell lexer tests passed!"),
        Err(e) => panic!("PowerShell lexer tests failed: {}", e),
    }
}
