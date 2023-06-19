#![feature(new_range_api)]

use oak_core::LexerTester;
use oak_twig::{lexer::TwigLexer, language::TwigLanguage};

#[test]
fn test_twig_lexer() {
    let config = TwigLanguage::default();
    let lexer = TwigLexer::new(&config);
    let tester = LexerTester::new(lexer);
    tester.run("twig");
}