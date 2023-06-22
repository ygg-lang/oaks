use oak_kotlin::{KotlinLanguage, KotlinLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_kotlin_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = KotlinLanguage::default();
    let lexer = KotlinLexer::new(&language);
    let test_runner = LexerTester::new(tests).with_extension("kt").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
