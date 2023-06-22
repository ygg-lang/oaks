use core::range::Range;
use serde::{Deserialize, Serialize};

/// JSON AST 根节点
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonRoot {
    pub value: JsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    Boolean(JsonBoolean),
    Null(JsonNull),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonObject {
    pub fields: Vec<JsonField>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonField {
    pub name: JsonString,
    pub value: JsonValue,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonArray {
    pub elements: Vec<JsonValue>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonString {
    pub value: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonNumber {
    pub value: f64,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonBoolean {
    pub value: bool,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonNull {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
