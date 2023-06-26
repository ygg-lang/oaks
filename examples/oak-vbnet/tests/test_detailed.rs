use oak_vbnet::parse;
use std::fs;

fn main() {
    let content = fs::read_to_string("test_vbnet_syntax.vb").expect("Failed to read test file");

    // Print the content around offset 713
    let start = std::cmp::max(0, 713 - 20);
    let end = std::cmp::min(content.len(), 713 + 20);
    println!("Content around offset 713:");
    println!("{:?}", &content[start..end]);

    // Parse and print result
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
