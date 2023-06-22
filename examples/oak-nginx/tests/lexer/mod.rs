use oak_nginx::{NginxLanguage, NginxLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_nginx_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = NginxLanguage::default();
    let lexer = NginxLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("conf").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
