#[test]
fn test_graphql_query_parsing() {
    use oak_core::{Lexer, LexerState, SourceText};
    use oak_graphql::{GraphQLLanguage, GraphQLLexer};

    let source = SourceText::new("query { user { name } }");
    let language = Box::leak(Box::new(GraphQLLanguage::default()));
    let lexer = GraphQLLexer::new(&language);

    let result = lexer.lex(&source);

    // Testing GraphQL query parsing
    println!("Testing GraphQL query parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    // Lexing should succeed
    let tokens = result.result.expect("Lexing should succeed");
    // Should parse at least one token
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}'", first_token.kind, token_text);

    // ✅ GraphQL query parsing test passed!
    println!("✅ GraphQL query parsing test passed!")
}
