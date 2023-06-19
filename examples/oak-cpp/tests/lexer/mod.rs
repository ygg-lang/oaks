#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_cpp::{CppLanguage, CppLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_cpp_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CppLanguage::default()));
    let lexer = CppLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("cpp").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CppLanguage, _>(lexer) {
        Ok(()) => println!("C++ lexer tests passed!"),
        Err(e) => panic!("C++ lexer tests failed: {}", e),
    }
}