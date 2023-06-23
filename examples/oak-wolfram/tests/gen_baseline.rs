#![feature(new_range_api)]
use oak_core::{Lexer, ParseSession, SourceText, source::Source};
use oak_wolfram::{WolframLanguage, WolframLexer};
use serde_json::json;
use std::fs;

fn main() {
    let source_path = "tests/lexer/basic.wl";
    let source_text = fs::read_to_string(source_path).expect("Failed to read source");
    let source = SourceText::new(source_text);
    let language = WolframLanguage::default();
    let lexer = WolframLexer::new(&language);
    let mut cache = ParseSession::default();
    let result = lexer.lex(&source, &[], &mut cache);

    let tokens = result.result.expect("Lexing failed");
    let token_data: Vec<_> = tokens
        .iter()
        .map(|t| {
            let text = source.get_text_in(t.span.clone()).to_string();
            json!({
                "kind": format!("{:?}", t.kind),
                "text": text,
                "start": t.span.start,
                "end": t.span.end
            })
        })
        .collect();

    let output = json!({
        "success": true,
        "count": tokens.len(),
        "tokens": token_data,
        "errors": []
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
