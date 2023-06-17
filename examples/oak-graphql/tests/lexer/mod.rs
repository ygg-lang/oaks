use oak_core::helpers::LexerTester;
use oak_graphql::{GraphqlLanguage, GraphqlLexer};
use std::path::Path;

#[test]
fn test_graphql_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = GraphqlLexer::new(&GraphqlLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("graphql");
    match test_runner.run_tests::<GraphqlLanguage, _>(lexer) {
        Ok(()) => println!("GraphQL lexer tests passed!"),
        Err(e) => panic!("GraphQL lexer tests failed: {}", e),
    }
}