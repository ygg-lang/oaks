use oak_testing::lexing::LexerTester;
use oak_verilog::{VerilogLanguage, VerilogLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_verilog_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = VerilogLanguage {};
    let lexer = VerilogLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("v").with_timeout(Duration::from_secs(5));

    match test_runner.run_tests(&lexer) {
        Ok(()) => println!("Verilog lexer tests passed!"),
        Err(e) => panic!("Verilog lexer tests failed: {}", e),
    }
}
