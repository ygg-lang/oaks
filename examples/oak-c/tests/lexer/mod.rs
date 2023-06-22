use oak_c::{CLanguage, CLexer};
use oak_core::helpers::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_c_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CLanguage::default()));
    let lexer = CLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("c").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CLanguage, _>(&lexer) {
        Ok(()) => println!("C lexer tests passed!"),
        Err(e) => panic!("C lexer tests failed: {}", e),
    }
}
