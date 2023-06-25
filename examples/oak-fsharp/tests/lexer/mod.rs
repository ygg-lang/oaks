#[test]
fn test_fsharp_function_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_fsharp::{FSharpLanguage, FSharpLexer};

    let source = SourceText::new("let add x y = x + y\nlet result = add 1 2");
    let language = FSharpLanguage::default();
    let lexer = FSharpLexer::new(&language);

    let result = lexer.lex(&source);

    // Testing F# function parsing
    println!("Testing F# function parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    // Lexing should succeed
    let tokens = result.result.expect("Lexing should succeed");
    // Should parse at least one token
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}'", first_token.kind, token_text);

    // ✅ F# function parsing test passed!
    println!("✅ F# function parsing test passed!")
}
