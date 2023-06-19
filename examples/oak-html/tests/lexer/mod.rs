#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_html::{HtmlLanguage, HtmlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_html_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HtmlLanguage::default()));
    let lexer = HtmlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML lexer tests passed!"),
        Err(e) => panic!("HTML lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HtmlLanguage::default()));
    let lexer = HtmlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML peek behavior tests passed!"),
        Err(e) => panic!("HTML peek behavior tests failed: {}", e),
    }
}

#[test]
fn test_tag_parsing() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HtmlLanguage::default()));
    let lexer = HtmlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML tag parsing tests passed!"),
        Err(e) => panic!("HTML tag parsing tests failed: {}", e),
    }
}

#[test]
fn test_comment_parsing() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HtmlLanguage::default()));
    let lexer = HtmlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML comment parsing tests passed!"),
        Err(e) => panic!("HTML comment parsing tests failed: {}", e),
    }
}

#[test]
fn test_entity_parsing() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HtmlLanguage::default()));
    let lexer = HtmlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("html").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HtmlLanguage, _>(lexer) {
        Ok(()) => println!("HTML entity parsing tests passed!"),
        Err(e) => panic!("HTML entity parsing tests failed: {}", e),
    }
}
