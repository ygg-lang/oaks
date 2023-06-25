use oak_core::{Lexer, parser::ParseSession};
use oak_dejavu::{DejavuLanguage, DejavuParser, ast::Item};

#[test]
fn test_annotation() {
    // let source = "
    // @specialize
    // @inline(always)
    // micro foo() {}
    // ";
    // let language = DejavuLanguage::default();
    // let parser = DejavuParser::new(&language);
    // let mut cache = ParseSession::<DejavuLanguage>::new(16);
    //
    // let result = parser.parse(&source, &[], &mut cache);
    // assert!(result.is_ok());
    //
    // let green_tree = result.unwrap();
    // let source_text = oak_core::source::SourceText::new(source);
    // let ast_root = parser.build_root(green_tree, &source_text).expect("Failed to build AST");
    //
    // assert_eq!(ast_root.items.len(), 1);
    // if let Item::Micro(m) = &ast_root.items[0] {
    // assert_eq!(m.annotations.len(), 2);
    // assert_eq!(m.annotations[0].name.name, "specialize");
    // assert_eq!(m.annotations[1].name.name, "inline");
    // assert_eq!(m.annotations[1].args.len(), 1);
    // assert_eq!(m.annotations[1].args[0].value, "always");
    // } else {
    // panic!("Expected Micro");
    // }
}
