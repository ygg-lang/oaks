use oak_core::helpers::ParserTester;
use oak_rust::{RustParser, language::RustLanguage};
use std::{path::Path, time::Duration};

#[test]
fn test_rust_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lang: &'static RustLanguage = Box::leak(Box::new(RustLanguage::default()));
    let parser: &'static RustParser = Box::leak(Box::new(RustParser::new(lang)));
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("rust").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RustLanguage, _>(parser) {
        Ok(()) => println!("Rust parser tests passed!"),
        Err(e) => panic!("Rust parser tests failed: {}", e),
    }
}
