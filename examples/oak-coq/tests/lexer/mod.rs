use oak_coq::{CoqLanguage, CoqLexer};
use oak_core::helpers::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_coq_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CoqLanguage::default()));
    let lexer = CoqLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("v").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CoqLanguage, _>(&lexer) {
        Ok(()) => println!("Coq lexer tests passed!"),
        Err(e) => panic!("Coq lexer tests failed: {}", e),
    }
}
