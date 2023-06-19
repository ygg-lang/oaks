use oak_core::helpers::LexerTester;
use oak_protobuf::{language::ProtobufLanguage, lexer::ProtobufLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_protobuf_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(ProtobufLanguage::default()));
    let lexer = ProtobufLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("proto").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ProtobufLanguage, _>(lexer) {
        Ok(()) => println!("Protobuf lexer tests passed!"),
        Err(e) => panic!("Protobuf lexer tests failed: {}", e),
    }
}
