#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_toml::{TomlLanguage, TomlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_toml_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(TomlLanguage::default()));
    let lexer = TomlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("toml").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<TomlLanguage, _>(lexer) {
        Ok(()) => println!("TOML lexer tests passed!"),
        Err(e) => panic!("TOML lexer tests failed: {}", e),
    }
}