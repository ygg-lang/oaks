#![feature(new_range_api)]
use core::range::Range;
use oak_core::SourceText;
use oak_diagnostic::{ConsoleEmitter, Diagnostic, Emitter, HtmlEmitter, LspEmitter, PlainTextEmitter};

#[test]
fn test_demo_diagnostic() {
    let source = SourceText::new("fn main() {\n    let x = 10\n}");

    let diag = Diagnostic::error("missing semicolon").with_code("E0001").with_label(Range { start: 26, end: 28 }, "expected ';' here").with_help("try adding a semicolon at the end of the line");

    println!("--- Console Output (Unicode) ---");
    let console = ConsoleEmitter { unicode: true }.render(&source, &diag);
    println!("{}", console);

    println!("--- Console Output (ASCII) ---");
    let console_ascii = ConsoleEmitter { unicode: false }.render(&source, &diag);
    println!("{}", console_ascii);

    println!("--- Plain Text Output ---");
    let plain = PlainTextEmitter { unicode: false }.render(&source, &diag);
    println!("{}", plain);

    println!("--- HTML Output ---");
    let html = HtmlEmitter.render(&source, &diag);
    println!("{}", html);

    println!("--- LSP Output ---");
    let lsp = LspEmitter.render(&source, &diag);
    println!("{}", lsp);

    println!("\n=== i18n Diagnostic ===");
    struct MyLocalizer;
    impl oak_diagnostic::Localizer for MyLocalizer {
        fn localize(&self, key: &str, args: &std::collections::HashMap<String, String>) -> String {
            match key {
                "error.missing_semicolon" => {
                    let mut msg = "missing semicolon".to_string();
                    if !args.is_empty() {
                        msg.push_str(&format!(" (args: {:?})", args))
                    }
                    msg
                }
                _ => key.to_string(),
            }
        }
    }

    let i18n_diag = Diagnostic::error("default message").with_code("E0001").with_i18n("error.missing_semicolon").with_arg("expected", ";").with_label(Range { start: 26, end: 28 }, "expected ';' here");

    println!("--- Localized Console Output ---");
    println!("{}", ConsoleEmitter { unicode: true }.render_localized(&source, &i18n_diag, Some(&MyLocalizer), None));

    println!("\n=== OakError Diagnostic ===");
    use oak_core::errors::OakError;
    let oak_err = OakError::syntax_error("expected expression", 16, None);
    let oak_diag = Diagnostic::from_provider(&oak_err, &source);
    println!("--- Console Output (from OakError) ---");
    println!("{}", ConsoleEmitter { unicode: true }.render(&source, &oak_diag));

    println!("\n=== Multi-line Diagnostic ===");
    let multi_diag = Diagnostic::error("unclosed function").with_label(Range { start: 0, end: 32 }, "function starts here and is never closed");

    println!("--- Console Output (Unicode) ---");
    println!("{}", ConsoleEmitter { unicode: true }.render(&source, &multi_diag));

    println!("--- Console Output (ASCII) ---");
    println!("{}", ConsoleEmitter { unicode: false }.render(&source, &multi_diag))
}
