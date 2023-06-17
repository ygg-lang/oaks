use oak_actionscript::{ActionScriptLanguage, ActionScriptLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_actionscript_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ActionScriptLexer::new(&ActionScriptLanguage::default());
    // don't use `as` here to avoid confusion with ActionScript source files
    let test_runner = LexerTester::new(tests).with_extension("as");
    match test_runner.run_tests::<ActionScriptLanguage, _>(lexer) {
        Ok(()) => println!("ActionScript lexer tests passed!"),
        Err(e) => panic!("ActionScript lexer tests failed: {}", e),
    }
}
