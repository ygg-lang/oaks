use oak_core::{LexerCache, source::Source};

#[test]
fn test_simple_function_parsing() -> Result<(), oak_core::OakError> {
    use oak_core::{Lexer, Parser, SourceText};
    use oak_rust::{RustLanguage, RustLexer, RustParser};

    let source = SourceText::new("fn main() { println!(\"Hello, world!\") }");
    let language = RustLanguage::default();
    let parser = RustParser::new(&language);

    // First test the lexer.
    println!("Testing lexer:");
    let lexer = RustLexer::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);
    match &lex_output.result {
        Ok(tokens) => {
            // println!("Generated tokens: {:?}", tokens);
            println!("Token count: {}", tokens.len());

            cache.set_lex_output(lex_output.clone());

            // Parse using cache with tokens.
            let parse_output = parser.parse(&source, &[], &mut cache);

            println!("Testing simple function parsing:");
            println!("Source code: '{}'", (&source).get_text_from(0));
            match &parse_output.result {
                Ok(root) => {
                    println!("Parse result: {:?}", root);
                    println!("✅ Simple function parsing test passed!")
                }
                Err(e) => {
                    println!("❌ Parsing failed: {:?}", e);
                    return Err(e.clone());
                }
            }
        }
        Err(e) => {
            println!("❌ Lexing failed: {:?}", e);
            return Err(e.clone());
        }
    }
    Ok(())
}
