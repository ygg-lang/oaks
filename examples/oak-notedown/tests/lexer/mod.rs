use oak_diagnostic::testing::lexing::LexerTester;
use oak_notedown::{NoteLanguage as NotedownLanguage, NoteLexer as NotedownLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_notedown_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(NotedownLanguage::default()));
    let lexer = NotedownLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("nd").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<NotedownLanguage, _>(&lexer) {
        Ok(()) => println!("Notedown lexer tests passed!"),
        Err(e) => panic!("Notedown lexer tests failed: {}", e),
    }
}
