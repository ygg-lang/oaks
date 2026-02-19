use oak_vbnet::{VbNetLanguage, parse};

fn main() {
    // Read the test file
    let content = std::fs::read_to_string("test_vbnet_syntax.vb").expect("Failed to read test file");

    // Create a VB.NET language instance
    let language = VbNetLanguage::default();

    // Parse the content
    match parse(&language, &content) {
        Ok(root) => println!("Parsing successful! Root node: {:?}", root),
        Err(error) => println!("Parsing failed: {:?}", error),
    }
}
