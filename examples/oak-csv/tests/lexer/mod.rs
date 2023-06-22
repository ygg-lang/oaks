use oak_core::helpers::LexerTester;
use oak_csv::{CsvLanguage, CsvLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_csv_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CsvLanguage::default()));
    let lexer = CsvLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("csv").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CsvLanguage, _>(&lexer) {
        Ok(()) => println!("CSV lexer tests passed!"),
        Err(e) => panic!("CSV lexer tests failed: {}", e),
    }
}
