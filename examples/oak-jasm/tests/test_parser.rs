use oak_core::{Builder, SourceText, parser::ParseSession};
use oak_jasm::{JasmBuilder, JasmLanguage};

#[test]
fn test_jasm_builder_smoke() {
    let source = r#".class public HelloWorld
.super java/lang/Object
.end class
"#;
    let source_text = SourceText::new(source);
    let language = JasmLanguage::standard();
    let builder = JasmBuilder::new(&language);
    let mut cache = ParseSession::<JasmLanguage>::new(16);
    let result = builder.build(&source_text, &[], &mut cache);
    // Accept parse success or failure; this only verifies the API compiles and runs.
    let _ = result.result;
}
