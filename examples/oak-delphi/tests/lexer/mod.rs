use oak_core::helpers::LexerTester;
use oak_delphi::{DelphiLanguage, DelphiLexer};
use std::path::Path;

#[test]
fn test_delphi_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DelphiLexer::new(&DelphiLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("pas");
    match test_runner.run_tests::<DelphiLanguage, _>(lexer) {
        Ok(()) => println!("Delphi lexer tests passed!"),
        Err(e) => panic!("Delphi lexer tests failed: {}", e),
    }
}