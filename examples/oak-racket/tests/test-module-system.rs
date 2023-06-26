use oak_core::{Builder, parser::ParseSession, source::SourceText};
use oak_racket::{RacketBuilder, RacketLanguage};

fn main() {
    // Read the test file
    let test_content = include_str!("test-module.rkt");
    let source = SourceText::new(test_content.to_string());

    // Create language and builder
    let language = RacketLanguage::new();
    let builder = RacketBuilder::new(&language);

    // Build the AST
    let mut cache = ParseSession::<RacketLanguage>::default();
    let result = builder.build(&source, &[], &mut cache);

    match result.result {
        Ok(ast) => {
            println!("Successfully parsed the file!");
            println!("Number of expressions: {}", ast.expressions.len());

            // Print each expression
            for (i, expr) in ast.expressions.iter().enumerate() {
                println!("Expression {}: {:?}", i, expr);
            }
        }
        Err(e) => {
            println!("Error parsing the file: {:?}", e);
            for diag in result.diagnostics {
                println!("Diagnostic: {:?}", diag);
            }
        }
    }
}
