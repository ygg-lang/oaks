use oak_vampire::{VampireLanguage, VampireLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;
use std::time::Duration;

#[test]
fn test_vampire_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(VampireLanguage));
    let lexer = VampireLexer::new(language);
    let test_runner = LexerTester::new(tests)
        .with_extension("tptp")
        .with_timeout(Duration::from_secs(30));
    match test_runner.run_tests::<VampireLanguage, _>(&lexer) {
        Ok(()) => println!("Vampire lexer tests passed!"),
        Err(e) => panic!("Vampire lexer tests failed: {}", e),
    }
}