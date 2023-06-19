use oak_scss::{language::ScssLanguage, lexer::ScssLexer};

#[test]
fn test_scss_lexer() {
    let config = ScssLanguage::default();
    let _lexer = ScssLexer::new(&config);
}
