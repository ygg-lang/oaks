#![feature(new_range_api)]

use oak_core::LexerTester;
use oak_stylus::{lexer::StylusLexer, language::StylusLanguage};

#[test]
fn test_stylus_lexer() {
    let config = StylusLanguage::default();
    let lexer = StylusLexer::new(&config);
    let tester = LexerTester::new(lexer);
    tester.run("stylus");
}