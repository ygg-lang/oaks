#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_vala::{ValaLanguage, ValaLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_vala_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ValaLanguage::default()));
    let lexer = ValaLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("vala").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ValaLanguage, _>(lexer) {
        Ok(()) => println!("Vala lexer tests passed!"),
        Err(e) => panic!("Vala lexer tests failed: {}", e),
    }
}