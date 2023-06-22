// Test files module

use oak_rust::{RustBuilder, RustLanguage, RustLexer, RustParser};
use oak_testing::{building::BuilderTester, lexing::LexerTester, parsing::ParserTester};
use std::{path::Path, time::Duration};

#[test]
fn test_rust_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}

#[test]
fn test_rust_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RustLanguage::default();
    let parser = RustParser::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = ParserTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}

#[test]
fn test_rust_builder() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RustLanguage::default();
    let builder = RustBuilder::new(language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = BuilderTester::new(here.join("tests/files")).with_extension("rust").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&builder)
}
