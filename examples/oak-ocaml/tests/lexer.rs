use oak_core::helpers::LexerTester;
use oak_ocaml::{OCamlLanguage, OCamlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_ocaml_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(OCamlLanguage));
    let lexer = OCamlLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ml").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<OCamlLanguage, _>(&lexer) {
        Ok(()) => println!("OCaml lexer tests passed!"),
        Err(e) => panic!("OCaml lexer tests failed: {}", e),
    }
}
