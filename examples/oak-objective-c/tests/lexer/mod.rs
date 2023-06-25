use oak_core::{Language, Lexer, NoLexerCache};
use oak_objective_c::{ObjectiveCLanguage, ObjectiveCLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_objective_c_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ObjectiveCLanguage::default()));
    let lexer = ObjectiveCLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("m").with_timeout(Duration::from_secs(5));

    if std::env::var("UPDATE_EXPECT").is_ok() {
        let input_path = here.join("tests/lexer/basic.m");
        let output_path = here.join("tests/lexer/basic.m.lexed.json");
        let source = std::fs::read_to_string(&input_path).unwrap();
        let mut cache = NoLexerCache::default();
        let output = lexer.lex(source.as_str(), &[], &mut cache);
        let json = serde_json::to_string_pretty(&output).unwrap();
        std::fs::write(output_path, json).unwrap();
        println!("Updated expected output");
        return;
    }

    match test_runner.run_tests::<ObjectiveCLanguage, _>(&lexer) {
        Ok(()) => println!("Objective-C lexer tests passed!"),
        Err(e) => panic!("Objective-C lexer tests failed: {}", e),
    }
}
