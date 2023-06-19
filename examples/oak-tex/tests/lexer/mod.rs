use oak_core::helpers::LexerTester;
use oak_tex::{language::TexLanguage, lexer::TexLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_tex_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(TexLanguage::standard()));
    let lexer = TexLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("tex").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<TexLanguage, _>(lexer) {
        Ok(()) => println!("TeX lexer tests passed!"),
        Err(e) => panic!("TeX lexer tests failed: {}", e),
    }
}
