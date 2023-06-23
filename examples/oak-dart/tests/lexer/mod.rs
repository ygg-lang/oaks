// #![feature(new_range_api)]

use oak_dart::{DartLanguage, DartLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_dart_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DartLanguage::default()));
    let lexer = DartLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("dart").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
