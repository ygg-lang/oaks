use oak_core::helpers::LexerTester;
use oak_valkyrie::{language::ValkyrieLanguage, lexer::ValkyrieLexer};
use std::path::Path;

#[test]
fn test_valkyrie_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let d = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&d);
    // don't use `as` here to avoid confusion with ActionScript source files
    let test_runner = LexerTester::new(tests).with_extension("valkyrie");
    match test_runner.run_tests::<ValkyrieLanguage, _>(lexer) {
        Ok(()) => println!("ActionScript lexer tests passed!"),
        Err(e) => panic!("ActionScript lexer tests failed: {}", e),
    }
}
