use oak_core::{Lexer, SourceText};
use oak_matlab::{MatlabLanguage, MatlabLexer};

fn main() {
    let language = MatlabLanguage::default();
    let lexer = MatlabLexer::new(&language);
    let source = SourceText::new("x");
    
    println!("Starting lexing...");
    let result = lexer.lex(&source);
    println!("Lexing completed!");
    
    match result.result {
        Ok(tokens) => {
            println!("Found {} tokens", tokens.len());
            for token in tokens.iter() {
                println!("Token: {:?}", token)
            }
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}
