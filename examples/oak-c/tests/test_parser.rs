use oak_c::{CParser, language::CLanguage};
use oak_core::{Parser, Source, TextEdit, parser::ParseSession};
use std::rc::Rc;

fn main() {
    let code = include_str!("test_simple.c");
    let language = CLanguage::default();
    let parser = CParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::<CLanguage>::default();
    let source = code;
    let edits = &[];

    let result = parser.parse(source, edits, &mut cache);
    match result.result {
        Ok(output) => {
            println!("Parsing successful!");
            println!("Root node: {:?}", output);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
        }
    }
}
