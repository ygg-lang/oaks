use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_rbq::{RbqLanguage, RbqParser, ast::{RbqRoot, RbqLiteral, RbqExprKind}};

#[test]
fn test_rbq_ast_micro_return_type() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("micro add(a: i32, b: i32) -> i32;");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Micro(m) = &root.items[0] {
        assert_eq!(m.name, "add");
        assert_eq!(m.args.len(), 2);
        if let Some(oak_rbq::ast::RbqType::Named { path, .. }) = &m.return_type {
            assert_eq!(path, "i32");
        } else {
            panic!("Expected named return type");
        }
    }
    else {
        panic!("Expected micro")
    }
}

#[test]
fn test_rbq_ast_typed_literals() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("query = 123; query = \"hello\"; query = true;");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 3);
    
    if let oak_rbq::ast::RbqItem::Query(expr) = &root.items[0] {
        if let RbqExprKind::Literal(RbqLiteral::Number(n)) = &expr.kind {
            assert_eq!(n, "123");
        } else { panic!("Expected Number literal"); }
    }

    if let oak_rbq::ast::RbqItem::Query(expr) = &root.items[1] {
        if let RbqExprKind::Literal(RbqLiteral::String(s)) = &expr.kind {
            assert_eq!(s, "\"hello\"");
        } else { panic!("Expected String literal"); }
    }

    if let oak_rbq::ast::RbqItem::Query(expr) = &root.items[2] {
        if let RbqExprKind::Literal(RbqLiteral::Boolean(b)) = &expr.kind {
            assert_eq!(*b, true);
        } else { panic!("Expected Boolean literal"); }
    }
}

#[test]
fn test_rbq_ast_namespace_annotations() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("@meta(version=\"1.0\") namespace App { @api struct User {} }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Namespace(ns) = &root.items[0] {
        let _ = &ns.annotations; // Force check
        assert_eq!(ns.annotations.len(), 1);
        assert_eq!(ns.annotations[0].name, "meta");
        
        assert_eq!(ns.items.len(), 1);
        if let oak_rbq::ast::RbqItem::Struct(s) = &ns.items[0] {
            assert_eq!(s.annotations.len(), 1);
            assert_eq!(s.annotations[0].name, "api");
        }
    }
}

#[test]
fn test_rbq_ast_complex_trait() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("trait Auth { user_id: i32; login: micro(token: string) -> bool; }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Trait(t) = &root.items[0] {
        assert_eq!(t.items.len(), 2);
        // First item is a field
        if let oak_rbq::ast::RbqTraitItem::Field(f) = &t.items[0] {
            assert_eq!(f.name, "user_id");
        } else { panic!("Expected field"); }
        
        // Second item is a method (micro)
        if let oak_rbq::ast::RbqTraitItem::Method(m) = &t.items[1] {
            assert_eq!(m.name, "login");
            if let Some(oak_rbq::ast::RbqType::Named { path, .. }) = &m.return_type {
                assert_eq!(path, "bool");
            } else {
                panic!("Expected named return type");
            }
        } else { panic!("Expected method"); }
    }
}

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
        if let oak_rbq::ast::RbqType::Named { path, .. } = &s.fields[0].type_ref {
            assert_eq!(path, "i32");
        } else {
            panic!("Expected named type");
        }
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
        if let oak_rbq::ast::RbqType::Named { path, .. } = &s.fields[0].type_ref {
            assert_eq!(path, "utf8");
        } else {
            panic!("Expected named type");
        }
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

#[test]
fn test_rbq_ast_import() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("use std.io;");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Import(imp) = &root.items[0] {
        assert_eq!(imp.path, "std.io");
    }
    else {
        panic!("Expected import")
    }
}

#[test]
fn test_rbq_ast_trait() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("trait Printable { print: micro(); }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Trait(t) = &root.items[0] {
        assert_eq!(t.name, "Printable");
        assert_eq!(t.items.len(), 1);
    }
    else {
        panic!("Expected trait")
    }
}

#[test]
fn test_rbq_ast_complex_type() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source = SourceText::new("struct Data { items: &List<i32>? }");

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let root = RbqRoot::lower(red, source.text());

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Struct(s) = &root.items[0] {
        let field = &s.fields[0];
        if let oak_rbq::ast::RbqType::Named { path, generic_args, is_physical_ptr, is_optional } = &field.type_ref {
            assert_eq!(path, "List");
            assert!(is_physical_ptr);
            assert!(is_optional);
            assert_eq!(generic_args.len(), 1);
            if let oak_rbq::ast::RbqType::Named { path: arg_path, .. } = &generic_args[0] {
                assert_eq!(arg_path, "i32");
            } else {
                panic!("Expected named generic arg");
            }
        } else {
            panic!("Expected named type");
        }
    }
    else {
        panic!("Expected struct")
    }
}
