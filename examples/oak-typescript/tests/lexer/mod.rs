#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_typescript::{language::TypeScriptLanguage, lexer::TypeScriptLexer};
use std::sync::LazyLock;

static CONFIG: LazyLock<TypeScriptLanguage> = LazyLock::new(|| TypeScriptLanguage::standard());

#[test]
fn test_typescript_lexer() {
    let lexer = TypeScriptLexer::new(&CONFIG);
    let test_dir = std::env::current_dir().unwrap().join("tests").join("lexer");
    let tester = LexerTester::new(test_dir).with_extension("ts").with_timeout(std::time::Duration::from_secs(5));
    tester.run_tests(lexer).unwrap();
}
