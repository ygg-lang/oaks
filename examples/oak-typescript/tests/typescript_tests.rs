use oak_core::{Builder, source::SourceText};
use oak_typescript::{TypeScriptBuilder, TypeScriptLanguage};

#[test]
fn test_parse_typescript() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::builder::session::BuilderSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        function add(a: number, b: number): number {
            return a + b;
        }
        add(1, 2);
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok());
}

#[test]
fn test_parse_typescript_async() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::builder::session::BuilderSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        async function fetchData(url: string) {
            return await fetch(url);
        }
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok());
}

#[test]
fn test_parse_typescript_arrow_function() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::builder::session::BuilderSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        const square = (x: number) => x * x;
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok());
}
