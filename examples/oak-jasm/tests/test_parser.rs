use oak_core::{BuilderCache, SourceText};
use oak_jasm::{JasmBuilder, JasmLanguage};

fn main() {
    // Read the test JASM file
    let source = std::fs::read_to_string("test.jasm").expect("Failed to read test.jasm");
    let source_text = SourceText::new(source);

    // Create a JASM builder
    let language = JasmLanguage::standard();
    let builder = JasmBuilder::new(&language);

    // Create a cache
    let mut cache = oak_core::builder::DefaultBuilderCache::new();

    // Build the AST
    let result = builder.build(&source_text, &[], &mut cache);

    match result.result {
        Ok(ast) => {
            println!("Successfully parsed JASM file!");
            println!("Class name: {}", ast.class.name);
            println!("Modifiers: {:?}", ast.class.modifiers);
            println!("Version: {:?}", ast.class.version);
            println!("Source file: {:?}", ast.class.source_file);
            println!("Fields: {}", ast.class.fields.len());
            println!("Methods: {}", ast.class.methods.len());

            for field in &ast.class.fields {
                println!("  Field: {:?} {:?}", field.modifiers, field.name_and_descriptor);
            }

            for method in &ast.class.methods {
                println!("  Method: {:?} {:?}", method.modifiers, method.name_and_descriptor);
                println!("    Stack size: {:?}", method.stack_size);
                println!("    Locals count: {:?}", method.locals_count);
                println!("    Instructions: {}", method.instructions.len());
            }
        }
        Err(error) => {
            println!("Failed to parse JASM file: {:?}", error);
        }
    }
}
