use oak_core::helpers::LexerTester;
use oak_d::{DLanguage, DLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_d_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DLanguage::default()));
    let lexer = DLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("d").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DLanguage, _>(&lexer) {
        Ok(()) => println!("D lexer tests passed!"),
        Err(e) => panic!("D lexer tests failed: {}", e),
    }
}
