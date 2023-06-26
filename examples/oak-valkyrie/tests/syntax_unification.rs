use oak_core::{Builder, Parser, SourceText};
use oak_valkyrie::{ValkyrieBuilder, ValkyrieLanguage, ValkyrieParser, ast::*};

#[test]
fn test_syntax_unification() {
    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);
    let builder = ValkyrieBuilder::new(&language);

    // 1. Object initialization style
    let source_str1 = "Point { x: 1.0, y: 2.0 }";
    println!("Source 1: {:?}, len={}", source_str1, source_str1.len());
    let source1 = SourceText::new(source_str1);
    let mut cache1 = oak_core::parser::ParseSession::default();
    let result1 = parser.parse(&source1, &[], &mut cache1);
    if let Ok(tree) = &result1.result {
        println!("Tree 1:\n{:#?}", tree)
    }
    assert!(result1.result.is_ok(), "Failed to parse object initialization: {:?}", result1.diagnostics);

    let built1 = builder.build(&source1, &[], &mut cache1);
    assert!(built1.result.is_ok(), "Failed to build object initialization: {:?}", built1.diagnostics);
    let ast1 = built1.result.unwrap();

    // Check if it's an Object in AST
    if let Item::Statement(Statement::ExprStmt { expr: Expr::Object { callee, fields, .. }, .. }) = &ast1.items[0] {
        if let Expr::Ident(ident) = callee.as_ref() {
            assert_eq!(ident.name, "Point")
        }
        else {
            panic!("Expected Ident callee, got {:?}", callee)
        }
        assert_eq!(fields.len(), 2)
    }
    else {
        panic!("Expected Expr::Object, got {:?}", ast1.items[0])
    }

    // 2. Trailing closure style
    let source2 = SourceText::new("run_task { print(1) }");
    let mut cache2 = oak_core::parser::ParseSession::default();
    let result2 = parser.parse(&source2, &[], &mut cache2);
    assert!(result2.result.is_ok(), "Failed to parse trailing closure: {:?}", result2.diagnostics);

    let built2 = builder.build(&source2, &[], &mut cache2);
    assert!(built2.result.is_ok(), "Failed to build trailing closure: {:?}", built2.diagnostics);
    let ast2 = built2.result.unwrap();

    if let Item::Statement(Statement::ExprStmt { expr: Expr::Object { callee, fields, .. }, .. }) = &ast2.items[0] {
        if let Expr::Ident(ident) = callee.as_ref() {
            assert_eq!(ident.name, "run_task")
        }
        else {
            panic!("Expected Ident callee, got {:?}", callee)
        }
        // Trailing closure style currently has no fields (just statements)
        // This may need adjustment based on how trailing closures are handled
        assert_eq!(fields.len(), 0)
    }
    else {
        panic!("Expected Expr::Object, got {:?}", ast2.items[0])
    }
}
