use oak_core::{Builder, source::SourceText};
use oak_rust::{RustBuilder, RustLanguage};

#[test]
fn test_builder_runnability() {
    let language = RustLanguage::default();
    let builder = RustBuilder::new(&language);
    let source = SourceText::new("fn main() { let x = 1 + 2 }".to_string());

    // 我们只需要验证它能跑通 build 流程，不崩溃即可
    // 因为目前的 parser 和 builder 还是简化版的
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let result = builder.build(&source, &[], &mut cache);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.items.len(), 1)
}
