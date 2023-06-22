use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_rbq::{RbqLanguage, RbqParser, ast::RbqRoot};

#[test]
fn test_span_with_whitespace() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let source_str = "@table(\"users\") struct Users { @key user_id: uuid; user_name: utf8; }";
    let source = SourceText::new(source_str);

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    assert_eq!(green.length(), source_str.len() as u32, "GreenNode length should match source length");

    let red = oak_core::tree::RedNode::new(green, 0);
    let root = RbqRoot::lower(red, source_str);

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Struct(s) = &root.items[0] {
        assert_eq!(s.name, "Users");
        assert_eq!(s.fields.len(), 2);
        assert_eq!(s.fields[0].name, "user_id");
        assert_eq!(s.fields[1].name, "user_name");

        // Verify annotation on struct
        assert_eq!(s.annotations.len(), 1);
        assert_eq!(s.annotations[0].name, "table");
    }
    else {
        panic!("Expected struct");
    }
}

#[test]
fn test_top_level_annotation_propagation() {
    let source = r#"
@table("users")
struct Users {
    @key user_id: uuid;
    user_name: utf8;
}
"#;
    let parser = RbqParser::new(&RbqLanguage);
    let mut session = ParseSession::default();
    let source_text = SourceText::new(source.to_string());
    let output = parser.parse(&source_text, &[], &mut session).unwrap();
    let root = RbqRoot::lower(output.result.unwrap(), source);

    assert_eq!(root.items.len(), 1);
    if let oak_rbq::ast::RbqItem::Struct(s) = &root.items[0] {
        assert_eq!(s.name, "Users");
        assert_eq!(s.annotations.len(), 1, "Should have 1 annotation (table)");
        assert_eq!(s.annotations[0].name, "table");
        assert_eq!(s.annotations[0].args, vec!["\"users\""]);

        assert_eq!(s.fields[0].name, "user_id");
        assert_eq!(s.fields[0].annotations.len(), 1);
        assert_eq!(s.fields[0].annotations[0].name, "key");
    }
    else {
        panic!("Expected struct item");
    }
}
