#![feature(new_range_api)]

use oak_core::LexerTester;
use oak_scheme::{lexer::SchemeLexer, language::SchemeLanguage};

#[test]
fn test_scheme_lexer() {
    let config = SchemeLanguage::default();
    let lexer = SchemeLexer::new(&config);
    let tester = LexerTester::new(lexer);
    tester.run("scheme");
}