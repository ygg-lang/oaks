use oak_core::helpers::LexerTester;
use oak_handlebars::{HandlebarsLanguage, HandlebarsLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_handlebars_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HandlebarsLanguage::default()));
    let lexer = HandlebarsLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("hbs").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HandlebarsLanguage, _>(&lexer) {
        Ok(()) => println!("Handlebars lexer tests passed!"),
        Err(e) => panic!("Handlebars lexer tests failed: {}", e),
    }
}
