use oak_core::{Lexer, Source, SourceText};
use oak_rust::{RustLanguage, RustLexer};

fn main() {
    let source = SourceText::new("fn add(x: i32, y: i32) -> i32 {");
    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);
    match result.result {
        Ok(tokens) => {
            println!("Tokens:");
            for (i, token) in tokens.iter().enumerate() {
                let text = source.get_text_in(token.span.clone());
                println!("Token {}: {:?}, text: '{}', span: {:?}", i, token.kind, text, token.span);
            }
        }
        Err(err) => println!("Lexing failed: {err}"),
    }
}
