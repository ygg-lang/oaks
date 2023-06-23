use oak_core::{Parser, parser::ParseSession, source::SourceText};
use oak_valkyrie::{ValkyrieLanguage, ValkyrieParser};

#[test]
fn test_micro_annotations() {
    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);
    let mut cache = ParseSession::default();

    let source = SourceText::new("@specialize @inline micro main() { let x = 42 }");
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok(), "Parsing failed: {:?}", result.diagnostics);

    let green_tree = result.result.unwrap();
    let source_text = SourceText::new(source.text().to_string());
    let ast_root = parser.build_root(green_tree, &source_text).expect("Failed to build AST");

    assert_eq!(ast_root.items.len(), 1);
    if let oak_valkyrie::ast::Item::Micro(m) = &ast_root.items[0] {
        assert_eq!(m.annotations.len(), 2);
        assert_eq!(m.annotations[0].name.name, "specialize");
        assert_eq!(m.annotations[1].name.name, "inline")
    }
    else {
        panic!("Expected a micro definition")
    }
}

#[test]
fn test_micro_annotations_with_args() {
    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);
    let mut cache = ParseSession::default();

    let source = SourceText::new("@specialize(1, 2) micro main() { let x = 42 }");
    let result = parser.parse(&source, &[], &mut cache);

    assert!(result.result.is_ok(), "Parsing failed: {:?}", result.diagnostics);

    let green_tree = result.result.unwrap();
    let source_text = SourceText::new(source.text().to_string());
    let ast_root = parser.build_root(green_tree, &source_text).expect("Failed to build AST");

    assert_eq!(ast_root.items.len(), 1);
    if let oak_valkyrie::ast::Item::Micro(m) = &ast_root.items[0] {
        assert_eq!(m.annotations.len(), 1);
        assert_eq!(m.annotations[0].name.name, "specialize");
        assert_eq!(m.annotations[0].args.len(), 2)
    }
    else {
        panic!("Expected a micro definition")
    }
}
