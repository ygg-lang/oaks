use oak_vbnet::parse;
use std::fs;

fn main() {
    let content = fs::read_to_string("test_vbnet_syntax.vb").expect("Failed to read test file");

    match parse(&content) {
        Ok(ast) => {
            println!("✅ Parsing succeeded!");
            println!("Root has {} items", ast.items.len());
        }
        Err(error) => {
            println!("❌ Parsing failed: {:?}", error);
        }
    }
}
