use oak_core::{Builder, SourceText, parser::ParseSession};
use oak_scss::{ScssBuilder, ScssLanguage};

#[test]
fn test_scss_ast_build_smoke() {
    let scss_code = "$primary: #333;\nbody { color: $primary; }\n";
    let language = ScssLanguage::default();
    let builder = ScssBuilder::new(&language);
    let source = SourceText::new(scss_code);
    let mut cache = ParseSession::<ScssLanguage>::new(16);
    let result = builder.build(&source, &[], &mut cache);
    let _ = result.result;
}
