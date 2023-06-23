use oak_actionscript::{ActionScriptLanguage, ActionScriptLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_actionscript_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ActionScriptLanguage::default()));
    let lexer = ActionScriptLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("as").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ActionScriptLanguage, _>(&lexer) {
        Ok(()) => println!("ActionScript lexer tests passed!"),
        Err(e) => panic!("ActionScript lexer tests failed: {}", e),
    }
}
