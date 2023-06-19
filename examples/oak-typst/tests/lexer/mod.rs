#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_typst::{language::TypstLanguage, lexer::TypstLexer};
use std::{sync::LazyLock, time::Duration};

static CONFIG: LazyLock<TypstLanguage> = LazyLock::new(|| TypstLanguage::standard());

#[test]
fn test_typst_lexer() {
    let lexer = TypstLexer::new(&CONFIG);
    let test_dir = std::env::current_dir().unwrap().join("tests").join("lexer");
    let tester = LexerTester::new(test_dir).with_extension("typ").with_timeout(Duration::from_secs(5));
    tester.run_tests(lexer).unwrap();
}
