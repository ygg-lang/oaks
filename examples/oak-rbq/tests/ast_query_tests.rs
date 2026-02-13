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
        let expr = if let RbqExprKind::Pipeline { base, .. } = &query_expr.kind { if let RbqExprKind::Binary { right, .. } = &base.kind { right } else { base } } else { query_expr };

        if let RbqExprKind::Closure { args, body } = &expr.kind {
            assert!(args.is_empty());
            assert_eq!(body.len(), 2);
        }
        else {
            panic!("Expected Closure, got {:?}", expr.kind);
        }
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
    if let RbqItem::Query(expr) = &root.items[0] {
        if let RbqExprKind::Closure { args, body } = &expr.kind {
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
    if let RbqItem::Query(expr) = &root.items[0] {
        if let RbqExprKind::Pipeline { base, steps } = &expr.kind {
            if let RbqExprKind::Identifier(id) = &base.kind {
                assert_eq!(id, "users");
            }
            else {
                panic!("Expected Identifier base, got {:?}", base.kind);
            }
            assert_eq!(steps.len(), 2);
            assert_eq!(steps[0].name, "filter");
            assert_eq!(steps[0].args.len(), 1);
            assert_eq!(steps[1].name, "sort");
            assert_eq!(steps[1].args.len(), 1);
        }
        else {
            panic!("Expected Pipeline, got {:?}", expr.kind);
        }
    }
}
