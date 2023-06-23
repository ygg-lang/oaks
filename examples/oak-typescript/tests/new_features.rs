use oak_core::{Builder, source::SourceText};
use oak_typescript::{TypeScriptBuilder, TypeScriptLanguage};

#[test]
fn test_parse_jsx() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::parser::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        const element = <div>Hello World</div>;
        const fragment = <><Component /></>;
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "JSX parsing failed: {:?}", output.result.err())
}

#[test]
fn test_parse_decorators() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::parser::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        @sealed
        class Greeter {
            @format("Hello, %s")
            greeting: string;

            constructor(message: string) {
                this.greeting = message
            }

            @log
            greet(@required name: string) {
                return "Hello, " + name
            }
        }
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "Decorators parsing failed: {:?}", output.result.err())
}

#[test]
fn test_parse_declare() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::parser::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        declare var process: any;
        declare function foo(): void;
        declare class Bar {}
        declare interface Baz {}
        declare enum Qux {}
        declare namespace NS {}
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "Declare parsing failed: {:?}", output.result.err())
}
