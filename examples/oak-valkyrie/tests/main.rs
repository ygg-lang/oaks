#![feature(new_range_api)]

mod builder;
mod lexer;
mod parser;

#[test]
fn ready() {
    println!("Valkyrie tests are ready!")
}

#[test]
fn test_valkyrie_integration() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer, ValkyrieParser};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);
    let _parser = ValkyrieParser::new(&language);

    // Test basic integration
    let source = SourceText::new("namespace Test { micro main() { let x = 42 } }");

    // Test lexer
    let tokens: Vec<_> = lexer.tokenize(&source).collect();
    assert!(!tokens.is_empty(), "Lexer should produce tokens");

    println!("Valkyrie integration test passed - {} tokens generated", tokens.len())
}

#[test]
fn test_error_handling_parsing() {
    use oak_core::{Builder, Parser, SourceText, parser::ParseSession};
    use oak_valkyrie::{ValkyrieBuilder, ValkyrieLanguage, ValkyrieParser};

    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);
    let builder = ValkyrieBuilder::new(&language);

    // Test 1: try { ... } .catch { ... }
    {
        let source = SourceText::new("try { raise 1 } .catch { e => resume 2 }");
        let mut session = ParseSession::new(1024);

        let result = parser.parse(&source, &[], &mut session);
        if result.has_errors() {
            panic!("Parse diagnostic: {:?}", result.diagnostics)
        }

        let built = builder.build(&source, &[], &mut session);
        if built.has_errors() {
            panic!("Build diagnostic: {:?}", built.diagnostics)
        }
        let ast = built.result.unwrap();
        println!("AST 1: {:?}", ast);

        let ast_str = format!("{:?}", ast);
        assert!(ast_str.contains("Resume"), "AST should contain Resume expression")
    }

    // Test 2: try Result<I32, Error> { ... } .catch { ... }
    {
        let source = SourceText::new("try Result<I32, Error> { raise 1 } .catch { e => 2 }");
        let mut session = ParseSession::new(1024);

        let result = parser.parse(&source, &[], &mut session);
        if result.has_errors() {
            panic!("Parse diagnostic: {:?}", result.diagnostics)
        }

        let built = builder.build(&source, &[], &mut session);
        if built.has_errors() {
            panic!("Build diagnostic: {:?}", built.diagnostics)
        }
        let ast = built.result.unwrap();
        println!("AST 2: {:?}", ast);

        let ast_str = format!("{:?}", ast);
        assert!(ast_str.contains("return_type: Some"), "AST should contain return_type for try-type syntax");
        assert!(ast_str.contains("Result"), "return_type should contain 'Result'")
    }

    // Test 3: Multiple .catch blocks
    {
        let source = SourceText::new("try { raise 1 } .catch { 1 => 2 } .catch { _ => 3 }");
        let mut session = ParseSession::new(1024);

        let result = parser.parse(&source, &[], &mut session);
        if result.has_errors() {
            panic!("Parse diagnostic: {:?}", result.diagnostics)
        }

        let built = builder.build(&source, &[], &mut session);
        if built.has_errors() {
            panic!("Build diagnostic: {:?}", built.diagnostics)
        }
        let ast = built.result.unwrap();
        println!("AST 3: {:?}", ast);

        let ast_str = format!("{:?}", ast);
        // Current implementation combines arms into a single Catch expression's arms vector
        assert!(ast_str.contains("arms: ["), "AST should contain arms")
    }

    // Test 4: Nested try-catch
    {
        let source = SourceText::new("try { try { raise 1 } .catch { _ => raise 2 } } .catch { _ => 3 }");
        let mut session = ParseSession::new(1024);

        let result = parser.parse(&source, &[], &mut session);
        if result.has_errors() {
            panic!("Parse diagnostic: {:?}", result.diagnostics)
        }

        let built = builder.build(&source, &[], &mut session);
        if built.has_errors() {
            panic!("Build diagnostic: {:?}", built.diagnostics)
        }
        let ast = built.result.unwrap();
        println!("AST 4: {:?}", ast)
    }
}
