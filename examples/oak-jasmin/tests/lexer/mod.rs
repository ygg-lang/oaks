use oak_diagnostic::testing::lexing::LexerTester;
use oak_jasmin::{JasminLanguage, JasminLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_jasmin_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(JasminLanguage::default()));
    let lexer = JasminLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("jasmin").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests(&lexer) {
        Ok(()) => println!("Jasmin lexer tests passed!"),
        Err(e) => panic!("Jasmin lexer tests failed: {}", e),
    }
}
