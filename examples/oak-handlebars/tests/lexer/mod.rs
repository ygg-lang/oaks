use oak_core::helpers::LexerTester;
use oak_handlebars::{HandlebarsLanguage, HandlebarsLexer};
use std::path::Path;

#[test]
fn test_handlebars_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = HandlebarsLexer::new(&HandlebarsLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("hbs");
    match test_runner.run_tests::<HandlebarsLanguage, _>(lexer) {
        Ok(()) => println!("Handlebars lexer tests passed!"),
        Err(e) => panic!("Handlebars lexer tests failed: {}", e),
    }
}
