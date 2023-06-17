use oak_core::helpers::LexerTester;
use oak_clojure::{ClojureLanguage, ClojureLexer};
use std::path::Path;

#[test]
fn test_clojure_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ClojureLexer::new(&ClojureLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("clj");
    match test_runner.run_tests::<ClojureLanguage, _>(lexer) {
        Ok(()) => println!("Clojure lexer tests passed!"),
        Err(e) => panic!("Clojure lexer tests failed: {}", e),
    }
}