#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_dart::{DartLanguage, DartLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_dart_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DartLanguage::default()));
    let lexer = DartLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("dart").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DartLanguage, _>(lexer) {
        Ok(()) => println!("Dart lexer tests passed!"),
        Err(e) => panic!("Dart lexer tests failed: {}", e),
    }
}