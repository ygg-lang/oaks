use oak_core::helpers::LexerTester;
use oak_cobol::{CobolLanguage, CobolLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_cobol_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CobolLanguage::default()));
    let lexer = CobolLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("cobol").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CobolLanguage, _>(&lexer) {
        Ok(()) => println!("COBOL lexer tests passed!"),
        Err(e) => panic!("COBOL lexer tests failed: {}", e),
    }
}