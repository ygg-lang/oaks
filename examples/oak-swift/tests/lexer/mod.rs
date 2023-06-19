#![feature(new_range_api)]

use oak_core::LexerTester;
use oak_swift::{lexer::SwiftLexer, language::SwiftLanguage};

#[test]
fn test_swift_lexer() {
    let config = SwiftLanguage::default();
    let lexer = SwiftLexer::new(&config);
    let tester = LexerTester::new(lexer);
    tester.run("swift");
}