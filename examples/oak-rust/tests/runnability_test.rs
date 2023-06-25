use oak_core::{Builder, source::SourceText};
use oak_rust::{RustBuilder, RustLanguage};

#[test]
fn test_builder_runnability() {
    let language = RustLanguage::default();
    let builder = RustBuilder::new(&language);
    let source = SourceText::new("fn main() { let x = 1 + 2 }".to_string());

    // We only need to verify that it can run through the build process without crashing
    // Because the current parser and builder are still simplified versions
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let result = builder.build(&source, &[], &mut cache);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.items.len(), 1)
}
