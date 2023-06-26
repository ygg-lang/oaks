use oak_core::source::SourceText;
use oak_markdown::{MarkdownLanguage, MarkdownLexer};
fn main() {
    let language = MarkdownLanguage::default();
    let lexer = MarkdownLexer::new(&language);

    let source = SourceText::new(include_str!("tests/lexer/basic.markdown"));
    let output = lexer.lex_internal(&source);

    match output.result {
        Ok(tokens) => {
            println!("Tokens count: {}", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("Token {}: {:?}", i, token);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    println!("Diagnostics: {:?}", output.diagnostics);
}
