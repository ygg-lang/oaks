// Test files module

use oak_core::helpers::{BuilderTester, LexerTester, ParserTester};
use oak_rust::{RustBuilder, RustLanguage, RustLexer, RustParser};
use std::{path::Path, time::Duration};

#[test]
fn test_rust_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RustLanguage::default()));
    let lexer = RustLexer::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RustLanguage, _>(&lexer) {
        Ok(()) => println!("Rust lexer tests passed!"),
        Err(e) => panic!("Rust lexer tests failed: {}", e),
    }
}

#[test]
fn test_rust_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RustLanguage::default()));
    let parser = RustParser::new(language);
    let _lexer = RustLexer::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = ParserTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RustLanguage, _>(&parser) {
        Ok(()) => println!("Rust files tests passed!"),
        Err(e) => panic!("Rust files tests failed: {}", e),
    }
}

#[test]
fn test_rust_builder() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RustLanguage::default()));
    let builder = RustBuilder::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = BuilderTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RustLanguage, _>(&builder) {
        Ok(()) => println!("Rust builder tests passed!"),
        Err(e) => panic!("Rust builder tests failed: {}", e),
    }
}
