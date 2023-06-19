#![feature(new_range_api)]

use oak_ada::{AdaLanguage, AdaLexer};
use oak_core::helpers::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_ada_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(AdaLanguage::default()));
    let lexer = AdaLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ada").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<AdaLanguage, _>(lexer) {
        Ok(()) => println!("Ada lexer tests passed!"),
        Err(e) => panic!("Ada lexer tests failed: {}", e),
    }
}
