use oak_core::helpers::LexerTester;
use oak_nginx::{NginxLanguage, NginxLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_nim_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(NginxLanguage::default()));
    let lexer = NginxLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("conf").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<NginxLanguage, _>(lexer) {
        Ok(()) => println!("Nginx lexer tests passed!"),
        Err(e) => panic!("Nginx lexer tests failed: {}", e),
    }
}
