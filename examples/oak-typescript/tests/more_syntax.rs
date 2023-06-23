use oak_core::Builder;
use oak_typescript::{ast::*, builder::TypeScriptBuilder, language::TypeScriptLanguage};

#[test]
fn test_parse_control_flow() {
    let language = TypeScriptLanguage::default();
    let builder = TypeScriptBuilder::new(&language);

    let source = "
        if (a) { b() } else { c() }
        while (true) { break }
        switch (x) { case 1: break; default: return }
        try { a() } catch (e) { b() } finally { c() }
    ";

    let mut cache = oak_core::parser::ParseSession::<TypeScriptLanguage>::default();
    let diagnostics = builder.build(&source, &[], &mut cache);
    assert!(diagnostics.result.is_ok());
    let root = diagnostics.result.unwrap();
    assert_eq!(root.statements.len(), 4);

    // Check IfStatement
    if let Statement::IfStatement(if_stmt) = &root.statements[0] {
        if let ExpressionKind::Identifier(_) = &*if_stmt.test.kind {
            // OK
        }
        else {
            panic!("Expected Identifier as if test, got {:?}", if_stmt.test)
        }
        assert!(if_stmt.alternate.is_some())
    }
    else {
        panic!("Expected IfStatement, got {:?}", root.statements[0])
    }

    // Check WhileStatement
    if let Statement::WhileStatement(while_stmt) = &root.statements[1] {
        if let ExpressionKind::BooleanLiteral(val) = &*while_stmt.test.kind { assert!(*val) } else { panic!("Expected BooleanLiteral as while test, got {:?}", while_stmt.test) }
    }
    else {
        panic!("Expected WhileStatement, got {:?}", root.statements[1])
    }

    // Check SwitchStatement
    if let Statement::SwitchStatement(switch_stmt) = &root.statements[2] {
        assert_eq!(switch_stmt.cases.len(), 2)
    }
    else {
        panic!("Expected SwitchStatement, got {:?}", root.statements[2])
    }

    // Check TryStatement
    if let Statement::TryStatement(try_stmt) = &root.statements[3] {
        assert!(try_stmt.handler.is_some());
        assert!(try_stmt.finalizer.is_some())
    }
    else {
        panic!("Expected TryStatement, got {:?}", root.statements[3])
    }
}

#[test]
fn test_parse_expressions() {
    let language = TypeScriptLanguage::default();
    let builder = TypeScriptBuilder::new(&language);

    let source = "
        let a = (x, y) => x + y;
        let b = [1, 2, 3];
        let c = { x: 1, y: 2 }
    ";

    let mut cache = oak_core::parser::ParseSession::<TypeScriptLanguage>::default();
    let diagnostics = builder.build(&source, &[], &mut cache);
    assert!(diagnostics.result.is_ok());
    let root = diagnostics.result.unwrap();
    assert_eq!(root.statements.len(), 3);

    // Check ArrowFunction
    if let Statement::VariableDeclaration(var) = &root.statements[0] {
        if let Some(expr) = &var.value {
            if let ExpressionKind::ArrowFunction { params, .. } = &*expr.kind { assert_eq!(params.len(), 2) } else { panic!("Expected ArrowFunction, got {:?}", var.value) }
        }
        else {
            panic!("Expected value in variable declaration")
        }
    }
    else {
        panic!("Expected VariableDeclaration, got {:?}", root.statements[0])
    }

    // Check ArrayLiteral
    if let Statement::VariableDeclaration(var) = &root.statements[1] {
        if let Some(expr) = &var.value {
            if let ExpressionKind::ArrayLiteral { elements } = &*expr.kind { assert_eq!(elements.len(), 3) } else { panic!("Expected ArrayLiteral, got {:?}", var.value) }
        }
        else {
            panic!("Expected value in variable declaration")
        }
    }
    else {
        panic!("Expected VariableDeclaration, got {:?}", root.statements[1])
    }

    // Check ObjectLiteral
    if let Statement::VariableDeclaration(var) = &root.statements[2] {
        if let Some(expr) = &var.value {
            if let ExpressionKind::ObjectLiteral { properties } = &*expr.kind { assert_eq!(properties.len(), 2) } else { panic!("Expected ObjectLiteral, got {:?}", var.value) }
        }
        else {
            panic!("Expected value in variable declaration")
        }
    }
    else {
        panic!("Expected VariableDeclaration, got {:?}", root.statements[2])
    }
}
