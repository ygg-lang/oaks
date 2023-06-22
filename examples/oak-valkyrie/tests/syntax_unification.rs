use oak_core::{Parser, SourceText};
use oak_valkyrie::{ValkyrieLanguage, ValkyrieParser, ast::*};

#[test]
fn test_syntax_unification() {
    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);

    // 1. Object initialization style
    let source_str1 = "Point { x = 1.0, y = 2.0 }";
    println!("Source 1: {:?}, len={}", source_str1, source_str1.len());
    let source1 = SourceText::new(source_str1);
    let mut cache1 = oak_core::parser::ParseSession::default();
    let result1 = parser.parse(&source1, &[], &mut cache1);
    if let Ok(tree) = result1.result {
        println!("Tree 1:\n{:#?}", tree);
    }
    assert!(result1.result.is_ok(), "Failed to parse object initialization: {:?}", result1.diagnostics);

    let green1 = result1.result.unwrap();
    let ast1 = parser.build_root(green1, &source1).expect("Failed to build AST for object initialization");

    // Check if it's an ApplyBlock/Object in AST
    if let Item::Statement(Statement::ExprStmt { expr: Expr::Object { callee, block, .. }, .. }) = &ast1.items[0] {
        if let Expr::Ident(ident) = callee.as_ref() {
            assert_eq!(ident.name, "Point");
        }
        else {
            panic!("Expected Ident callee, got {:?}", callee);
        }
        assert_eq!(block.statements.len(), 2);
    }
    else {
        panic!("Expected Expr::Object, got {:?}", ast1.items[0]);
    }

    // 2. Trailing closure style
    let source2 = SourceText::new("run_task { print(1) }");
    let mut cache2 = oak_core::parser::ParseSession::default();
    let result2 = parser.parse(&source2, &[], &mut cache2);
    assert!(result2.result.is_ok(), "Failed to parse trailing closure: {:?}", result2.diagnostics);

    let green2 = result2.result.unwrap();
    let ast2 = parser.build_root(green2, &source2).expect("Failed to build AST for trailing closure");

    if let Item::Statement(Statement::ExprStmt { expr: Expr::Object { callee, block, .. }, .. }) = &ast2.items[0] {
        if let Expr::Ident(ident) = callee.as_ref() {
            assert_eq!(ident.name, "run_task");
        }
        else {
            panic!("Expected Ident callee, got {:?}", callee);
        }
        assert_eq!(block.statements.len(), 1);
    }
    else {
        panic!("Expected Expr::Object, got {:?}", ast2.items[0]);
    }
}
