use oak_c::{CParser, language::CLanguage};
use oak_core::{Parser, Source, TextEdit, parser::ParseSession};
use std::rc::Rc;

fn main() {
    let code = r#"
    int main() {
        int x = 10;
        int y = 20;
        return x + y;
    }
    
    struct Point {
        int x;
        int y;
    };
    
    int add(int a, int b) {
        return a + b;
    }
    "#;

    let language = CLanguage::default();
    let parser = CParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::<CLanguage>::default();
    let source = code;
    let edits = &[];

    println!("Parsing C code...");
    let result = parser.parse(source, edits, &mut cache);
    match result.result {
        Ok(output) => {
            println!("Parsing successful!");
            println!("Root node kind: {:?}", output.kind());
            println!("Number of children: {:?}", output.children().len());
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
        }
    }
    println!("Parsing completed.");
}
