use oak_core::helpers::LexerTester;
use oak_kotlin::{KotlinLanguage, KotlinLexer};
use std::path::Path;

#[test]
fn test_kotlin_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let binding = KotlinLanguage::default();
    let lexer = KotlinLexer::new(&binding);
    let test_runner = LexerTester::new(tests).with_extension("kt");
    match test_runner.run_tests::<KotlinLanguage, _>(lexer) {
        Ok(()) => println!("Kotlin lexer tests passed!"),
        Err(e) => panic!("Kotlin lexer tests failed: {}", e),
    }
}
