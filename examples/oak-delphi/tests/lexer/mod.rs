#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_delphi::{DelphiLanguage, DelphiLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_delphi_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DelphiLanguage::default()));
    let lexer = DelphiLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer"))
        .with_extension("pas")
        .with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DelphiLanguage, _>(lexer) {
        Ok(()) => println!("Delphi lexer tests passed!"),
        Err(e) => panic!("Delphi lexer tests failed: {}", e),
    }
}