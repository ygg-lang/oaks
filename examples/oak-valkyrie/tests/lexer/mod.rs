#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_valkyrie::{language::ValkyrieLanguage, lexer::ValkyrieLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_valkyrie_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(ValkyrieLanguage::default()));
    let lexer = ValkyrieLexer::new(language);
    // don't use `as` here to avoid confusion with ActionScript source files
    let test_runner = LexerTester::new(tests).with_extension("valkyrie").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ValkyrieLanguage, _>(lexer) {
        Ok(()) => println!("Valkyrie lexer tests passed!"),
        Err(e) => panic!("Valkyrie lexer tests failed: {}", e),
    }
}
