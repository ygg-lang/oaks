use oak_core::{Lexer, ParseSession, source::SourceText};
use oak_vhdl::{VhdlLanguage, VhdlLexer};

#[test]
fn test_vhdl_lexer_simple() {
    let lexer = VhdlLexer::new(&VhdlLanguage {});
    let source = SourceText::new("entity test is end entity;");
    let mut session = ParseSession::<VhdlLanguage>::new(16);
    let result = lexer.lex(&source, &[], &mut session);

    // Check if tokens were successfully generated
    match &result.result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Lexer should produce tokens");
            println!("Generated {} tokens", tokens.len());

            // Print the first few tokens for debugging
            for (i, token) in tokens.iter().take(5).enumerate() {
                println!("Token {}: {:?}", i, token)
            }
        }
        Err(e) => {
            panic!("Lexer failed with error: {}", e)
        }
    }

    // Check for diagnostics
    if !result.diagnostics.is_empty() {
        println!("Diagnostics: {:?}", result.diagnostics)
    }
}
