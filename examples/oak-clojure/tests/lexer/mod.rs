use oak_core::helpers::LexerTester;
use oak_clojure::{ClojureLanguage, ClojureLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_clojure_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ClojureLanguage::default()));
    let lexer = ClojureLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("clj").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ClojureLanguage, _>(&lexer) {
        Ok(()) => println!("Clojure lexer tests passed!"),
        Err(e) => panic!("Clojure lexer tests failed: {}", e),
    }
}