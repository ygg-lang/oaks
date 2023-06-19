#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_scala::{ScalaLanguage, lexer::ScalaLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_scala_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = ScalaLanguage::default();
    let lexer = ScalaLexer::new(&language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/lexer"))
        .with_extension("scala")
        .with_extension("dotty")
        .with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ScalaLanguage, _>(lexer) {
        Ok(()) => println!("Scala lexer tests passed!"),
        Err(e) => panic!("Scala lexer tests failed: {}", e),
    }
}
