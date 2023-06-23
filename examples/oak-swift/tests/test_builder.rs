use oak_core::{Builder, SourceText, builder::BuilderCache, parser::session::ParseSession};
use oak_swift::{SwiftBuilder, SwiftLanguage};

#[test]
fn test_basic_function() {
    let language = SwiftLanguage::default();
    let builder = SwiftBuilder::new(&language);
    let source = SourceText::new("func hello(name: String) {\n    return name\n}");
    let mut cache = ParseSession::default();
    let output = builder.build(&source, &[], &mut cache);

    assert!(output.result.is_ok());
    let ast = output.result.unwrap();
    assert_eq!(ast.program.statements.len(), 1);

    match &ast.program.statements[0] {
        oak_swift::ast::Statement::FunctionDef { name, parameters, .. } => {
            assert_eq!(name, "hello");
            assert_eq!(parameters.len(), 1);
            assert_eq!(parameters[0].name, "name");
            assert_eq!(parameters[0].type_annotation.name, "String")
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_variable_declaration() {
    let language = SwiftLanguage::default();
    let builder = SwiftBuilder::new(&language);
    let source = SourceText::new("let x = 42\nvar y = \"hello\"");
    let mut cache = ParseSession::default();
    let output = builder.build(&source, &[], &mut cache);

    assert!(output.result.is_ok());
    let ast = output.result.unwrap();
    assert_eq!(ast.program.statements.len(), 2);

    match &ast.program.statements[0] {
        oak_swift::ast::Statement::VariableDecl { name, is_mutable, .. } => {
            assert_eq!(name, "x");
            assert_eq!(*is_mutable, false)
        }
        _ => panic!("Expected variable declaration"),
    }

    match &ast.program.statements[1] {
        oak_swift::ast::Statement::VariableDecl { name, is_mutable, .. } => {
            assert_eq!(name, "y");
            assert_eq!(*is_mutable, true)
        }
        _ => panic!("Expected variable declaration"),
    }
}
