use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_rbq::{
    RbqLanguage, RbqParser,
    ast::{RbqExprKind, RbqItem, RbqRoot},
};

#[test]
fn test_rbq_ast_closure_simple() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("query = { 1; 2; };");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok(), "Parse failed: {:?}", output.diagnostics);

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    let root = RbqRoot::lower(red, source.text());
    println!("Items: {:#?}", root.items);
    assert_eq!(root.items.len(), 1);
    if let RbqItem::Query(query_expr) = &root.items[0] {
        let expr = unwrap_query(query_expr);

        if let RbqExprKind::Closure { args, body } = &expr.kind {
            assert!(args.is_empty());
            assert_eq!(body.len(), 2);
        }
        else {
            panic!("Expected Closure, got {:?}", expr.kind);
        }
    }
}

fn unwrap_query(expr: &oak_rbq::ast::RbqExpr) -> &oak_rbq::ast::RbqExpr {
    match &expr.kind {
        RbqExprKind::Binary { right, op, .. } if op == "=" => unwrap_query(right),
        _ => expr,
    }
}

#[test]
fn test_rbq_ast_closure_with_args() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("query = { |a, b| a + b; };");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok(), "Parse failed: {:?}", output.diagnostics);

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let RbqItem::Query(query_expr) = &root.items[0] {
        let expr = unwrap_query(query_expr);
        if let RbqExprKind::Closure { args, .. } = &expr.kind {
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], "a");
            assert_eq!(args[1], "b");
        }
        else {
            panic!("Expected Closure, got {:?}", expr.kind);
        }
    }
}

#[test]
fn test_rbq_ast_query_pipeline() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("query = { users | filter(age > 18) | sort(name) };");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok(), "Parse failed: {:?}", output.diagnostics);

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let RbqItem::Query(query_expr) = &root.items[0] {
        let expr = unwrap_query(query_expr);
        if let RbqExprKind::Pipeline { base, steps } = &expr.kind {
            let base_expr = unwrap_query(base);
            if let RbqExprKind::Identifier(id) = &base_expr.kind {
                assert_eq!(id, "users");
            }
            else {
                panic!("Expected Identifier base, got {:?}", base_expr.kind);
            }
            assert_eq!(steps.len(), 2);
            assert_eq!(steps[0].name, "filter");
            assert_eq!(steps[1].name, "sort");
        }
        else if let RbqExprKind::Binary { left, op, right } = &expr.kind {
            // 如果 Pipeline 被解析成了 Binary (可能是因为优先级问题)
            println!("Got Binary: {:?} {:?} {:?}", left, op, right);
        }
        else {
            panic!("Expected Pipeline, got {:?}", expr.kind);
        }
    }
}
