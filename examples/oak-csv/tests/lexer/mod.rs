use oak_core::helpers::LexerTester;
use oak_csv::{CsvLanguage, CsvLexer};
use std::path::Path;

#[test]
fn test_csv_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CsvLexer::new(&CsvLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("csv");
    match test_runner.run_tests::<CsvLanguage, _>(lexer) {
        Ok(()) => println!("CSV lexer tests passed!"),
        Err(e) => panic!("CSV lexer tests failed: {}", e),
    }
}