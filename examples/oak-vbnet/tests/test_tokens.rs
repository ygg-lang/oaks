use oak_core::{lexer::Lexer, source::StringSource};
use oak_vbnet::{VbNetLanguage, VbNetLexer, VbNetTokenType};
use std::fs;

fn main() {
    let content = fs::read_to_string("test_vbnet_syntax.vb").expect("Failed to read test file");
    let source = StringSource::new(&content);
    let language = VbNetLanguage::new();
    let mut lexer = VbNetLexer::new(&language);

    let mut tokens = Vec::new();
    let mut offset = 0;

    while offset < content.len() {
        let result = lexer.lex(&source, offset);
        match result {
            Ok((token, new_offset)) => {
                tokens.push((offset, new_offset, token));
                offset = new_offset;
            }
            Err(error) => {
                println!("Lexing error at offset {}: {:?}", offset, error);
                break;
            }
        }
    }

    // Find tokens around offset 713
    println!("Tokens around offset 713:");
    for (i, (start, end, token)) in tokens.iter().enumerate() {
        if start <= &713 && end >= &713 {
            println!("Token {}: start={}, end={}, type={:?}, text={:?}", i, start, end, token.kind, &content[*start..*end]);
        }
    }
}
