use oak_core::helpers::LexerTester;
use oak_idl::{IdlLanguage, IdlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_idl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(IdlLanguage::default()));
    let lexer = IdlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("idl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<IdlLanguage, _>(lexer) {
        Ok(()) => println!("IDL lexer tests passed!"),
        Err(e) => panic!("IDL lexer tests failed: {}", e),
    }
}
