#![feature(new_range_api)]
use core::range::Range;
use oak_core::source::ToSource;
use oak_json::ast::*;

#[test]
fn test_json_to_source() {
    let root = JsonRoot {
        value: JsonValue::Object(JsonObject {
            fields: vec![
                JsonField { name: JsonString { value: "name".to_string(), span: Range { start: 0, end: 0 } }, value: JsonValue::String(JsonString { value: "oak".to_string(), span: Range { start: 0, end: 0 } }), span: Range { start: 0, end: 0 } },
                JsonField { name: JsonString { value: "version".to_string(), span: Range { start: 0, end: 0 } }, value: JsonValue::Number(JsonNumber { value: 1.0, span: Range { start: 0, end: 0 } }), span: Range { start: 0, end: 0 } },
            ],
            span: Range { start: 0, end: 0 },
        }),
    };

    let source = root.to_source_string();
    assert_eq!(source, "{\"name\":\"oak\",\"version\":1}");
}

#[cfg(feature = "oak-pretty-print")]
#[test]
fn test_json_to_doc() {
    use oak_pretty_print::{AsDocument, FormatConfig};

    let root = JsonRoot {
        value: JsonValue::Object(JsonObject {
            fields: vec![JsonField { name: JsonString { value: "name".to_string(), span: Range { start: 0, end: 0 } }, value: JsonValue::String(JsonString { value: "oak".to_string(), span: Range { start: 0, end: 0 } }), span: Range { start: 0, end: 0 } }],
            span: Range { start: 0, end: 0 },
        }),
    };

    let doc = root.to_doc();
    let formatted = doc.render(FormatConfig::default());
    assert!(formatted.contains("\"name\": \"oak\""));
}
