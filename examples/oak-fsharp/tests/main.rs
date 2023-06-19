use oak_core::{Lexer, SourceText};
use oak_fsharp::{FSharpLanguage, FSharpLexer};

#[test]
fn test_fsharp_lexer_integration() {
    let source = SourceText::new("let add x y = x + y\nlet result = add 5 3");
    let language = Box::leak(Box::new(FSharpLanguage::default()));
    let lexer = FSharpLexer::new(language);

    let result = lexer.lex(&source);

    match result.result {
        Ok(tokens) => {
            println!("F# lexer tests passed! Parsed {} tokens", tokens.len());
            assert!(!tokens.is_empty(), "Should parse at least one token");
        }
        Err(e) => panic!("F# lexer tests failed: {:?}", e),
    }
}
