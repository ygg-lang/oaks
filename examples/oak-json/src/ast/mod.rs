use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
use serde::{Deserialize, Serialize};

#[cfg(feature = "pretty-print")]
use oak_pretty_print::{AsDocument, Document, FormatRule, doc as pp_doc};

/// JSON AST 根节点
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
pub struct JsonRoot {
    pub value: JsonValue,
}

impl ToSource for JsonRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.value.to_source(buffer);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    Boolean(JsonBoolean),
    Null(JsonNull),
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(&s.value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(n.value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(b.value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.as_f64().map(|f| f as u64)
    }

    pub fn as_array(&self) -> Option<&JsonArray> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&JsonObject> {
        match self {
            JsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(o) => o.get(key),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            JsonValue::Null(_) => "null".to_string(),
            JsonValue::Boolean(b) => b.value.to_string(),
            JsonValue::Number(n) => n.value.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s.value),
            JsonValue::Array(a) => {
                let elements: Vec<String> = a.elements.iter().map(|e| e.to_string()).collect();
                format!("[{}]", elements.join(","))
            }
            JsonValue::Object(o) => {
                let fields: Vec<String> = o.fields.iter().map(|f| format!("\"{}\":{}", f.name.value, f.value.to_string())).collect();
                format!("{{{}}}", fields.join(","))
            }
        }
    }
}

impl std::fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(JsonString { value: s.to_string(), span: (0..0).into() })
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(JsonString { value: s, span: (0..0).into() })
    }
}

impl From<f64> for JsonValue {
    fn from(f: f64) -> Self {
        JsonValue::Number(JsonNumber { value: f, span: (0..0).into() })
    }
}

impl From<u64> for JsonValue {
    fn from(u: u64) -> Self {
        JsonValue::Number(JsonNumber { value: u as f64, span: (0..0).into() })
    }
}

impl From<i32> for JsonValue {
    fn from(i: i32) -> Self {
        JsonValue::Number(JsonNumber { value: i as f64, span: (0..0).into() })
    }
}

impl From<i64> for JsonValue {
    fn from(i: i64) -> Self {
        JsonValue::Number(JsonNumber { value: i as f64, span: (0..0).into() })
    }
}

impl From<usize> for JsonValue {
    fn from(u: usize) -> Self {
        JsonValue::Number(JsonNumber { value: u as f64, span: (0..0).into() })
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Boolean(JsonBoolean { value: b, span: (0..0).into() })
    }
}

impl From<u32> for JsonValue {
    fn from(value: u32) -> Self {
        JsonValue::Number(JsonNumber { value: value as f64, span: (0..0).into() })
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        JsonValue::Number(JsonNumber { value: value as f64, span: (0..0).into() })
    }
}

impl<T: Into<JsonValue>> From<Option<T>> for JsonValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => JsonValue::Null(JsonNull { span: (0..0).into() }),
        }
    }
}

impl From<()> for JsonValue {
    fn from(_: ()) -> Self {
        JsonValue::Null(JsonNull { span: (0..0).into() })
    }
}

impl From<[JsonValue; 0]> for JsonValue {
    fn from(_: [JsonValue; 0]) -> Self {
        JsonValue::Array(JsonArray { elements: vec![], span: (0..0).into() })
    }
}

impl From<Vec<JsonValue>> for JsonValue {
    fn from(elements: Vec<JsonValue>) -> Self {
        JsonValue::Array(JsonArray { elements, span: (0..0).into() })
    }
}

impl From<std::collections::HashMap<String, JsonValue>> for JsonValue {
    fn from(fields: std::collections::HashMap<String, JsonValue>) -> Self {
        let mut fields_vec: Vec<JsonField> = fields.into_iter().map(|(k, v)| JsonField { name: JsonString { value: k, span: (0..0).into() }, value: v, span: (0..0).into() }).collect();
        // Sort fields by name for consistent output
        fields_vec.sort_by(|a, b| a.name.value.cmp(&b.name.value));
        JsonValue::Object(JsonObject { fields: fields_vec, span: (0..0).into() })
    }
}

impl<T: Into<JsonValue>> FromIterator<T> for JsonValue {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        JsonValue::Array(JsonArray { elements: iter.into_iter().map(Into::into).collect(), span: (0..0).into() })
    }
}

impl ToSource for JsonValue {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            JsonValue::Object(v) => v.to_source(buffer),
            JsonValue::Array(v) => v.to_source(buffer),
            JsonValue::String(v) => v.to_source(buffer),
            JsonValue::Number(v) => v.to_source(buffer),
            JsonValue::Boolean(v) => v.to_source(buffer),
            JsonValue::Null(v) => v.to_source(buffer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = if self.fields.is_empty() {
    pp_doc!("{}")
} else {
    pp_doc!(group [
        "{",
        indent [line, join(&self.fields, [",", line])],
        line,
        "}"
    ])
}))]
pub struct JsonObject {
    pub fields: Vec<JsonField>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl JsonObject {
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.fields.iter().find(|f| f.name.value == key).map(|f| &f.value)
    }
}

impl ToSource for JsonObject {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("{");
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                buffer.push(",");
            }
            field.to_source(buffer);
        }
        buffer.push("}");
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = [&self.name, ": ", &self.value]))]
pub struct JsonField {
    pub name: JsonString,
    pub value: JsonValue,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonField {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.name.to_source(buffer);
        buffer.push(":");
        self.value.to_source(buffer);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = if self.elements.is_empty() {
    pp_doc!("[]")
} else {
    pp_doc!(group [
        "[",
        indent [line, join(&self.elements, [",", line])],
        line,
        "]"
    ])
}))]
pub struct JsonArray {
    pub elements: Vec<JsonValue>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonArray {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("[");
        for (i, element) in self.elements.iter().enumerate() {
            if i > 0 {
                buffer.push(",");
            }
            element.to_source(buffer);
        }
        buffer.push("]");
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = format!("\"{}\"", self.value)))]
pub struct JsonString {
    pub value: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonString {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("\"");
        buffer.push(&self.value);
        buffer.push("\"");
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = self.value.to_string()))]
pub struct JsonNumber {
    pub value: f64,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonNumber {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.value.to_string());
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = if self.value { "true" } else { "false" }))]
pub struct JsonBoolean {
    pub value: bool,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonBoolean {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(if self.value { "true" } else { "false" });
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "pretty-print", derive(AsDocument))]
#[cfg_attr(feature = "pretty-print", oak(doc = "null"))]
pub struct JsonNull {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for JsonNull {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("null");
    }
}
