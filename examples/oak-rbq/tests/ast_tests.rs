use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_rbq::{RbqLanguage, RbqParser, ast::RbqRoot};

#[test]
fn test_rbq_ast_lowering() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("struct User { id: i32 }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Struct(s) = &root.items[0] {
        assert_eq!(s.name, "User");
        assert_eq!(s.fields.len(), 1);
        assert_eq!(s.fields[0].name, "id");
        assert_eq!(s.fields[0].type_ref, "i32")
    }
    else {
        panic!("Expected struct")
    }
}

#[test]
fn test_rbq_ast_utf8() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("struct User { name: utf8 }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Struct(s) = &root.items[0] {
        assert_eq!(s.name, "User");
        assert_eq!(s.fields.len(), 1);
        assert_eq!(s.fields[0].name, "name");
        assert_eq!(s.fields[0].type_ref, "utf8")
    }
    else {
        panic!("Expected struct")
    }
}

#[test]
fn test_rbq_ast_namespace() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("namespace App { struct User {} }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Namespace(ns) = &root.items[0] {
        assert_eq!(ns.name, "App");
        assert_eq!(ns.items.len(), 1)
    }
    else {
        panic!("Expected namespace")
    }
}
