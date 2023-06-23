use oak_core::{Builder, source::SourceText};
use oak_typescript::{TypeScriptBuilder, TypeScriptLanguage};

#[test]
fn test_parse_decorators() {
    let mut config = TypeScriptLanguage::standard();
    config.decorators = true;
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        @sealed
        class Greeter {
            @format("Hello, %s")
            greeting: string;

            constructor(message: string) {
                this.greeting = message
            }

            @enumerable(false)
            greet() {
                return "Hello, " + this.greeting
            }
        }
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "Decorators should be parsed correctly: {:?}", output.diagnostics)
}

#[test]
fn test_parse_declare() {
    let config = TypeScriptLanguage::standard();
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        declare var x: number;
        declare function f(): void;
        declare class C {}
        declare enum E {}
        declare namespace N {
            export var y: number
        }
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "Declare keyword should be parsed correctly: {:?}", output.diagnostics)
}

#[test]
fn test_parse_jsx() {
    let mut config = TypeScriptLanguage::standard();
    config.jsx = true;
    let builder = TypeScriptBuilder::new(&config);
    let mut session = oak_core::ParseSession::<TypeScriptLanguage>::default();

    let code = SourceText::new(
        r#"
        const element = (
            <div className="test">
                <h1>Hello</h1>
                <p>World</p>
                <SelfClosing />
                <Fragment>
                    <>
                        <span>Nested</span>
                    </>
                </Fragment>
            </div>
        );
    "#,
    );
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok(), "JSX should be parsed correctly: {:?}", output.diagnostics)
}
