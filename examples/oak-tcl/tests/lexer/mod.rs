use oak_core::helpers::LexerTester;
use oak_tcl::{TclLanguage, TclLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_tcl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(TclLanguage::standard()));
    let lexer = TclLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("tcl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<TclLanguage, _>(&lexer) {
        Ok(()) => println!("Tcl lexer tests passed!"),
        Err(e) => panic!("Tcl lexer tests failed: {}", e),
    }
}
