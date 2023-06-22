use oak_core::helpers::LexerTester;
use oak_nim::{NimLanguage, NimLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_nim_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(NimLanguage::default()));
    let lexer = NimLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("nim").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<NimLanguage, _>(&lexer) {
        Ok(()) => println!("Nim lexer tests passed!"),
        Err(e) => panic!("Nim lexer tests failed: {}", e),
    }
}
