#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// VON AST 根节点
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonRoot {
    pub value: VonValue,
}

impl ToSource for VonRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.value.to_source(buffer)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VonValue {
    Object(VonObject),
    Array(VonArray),
    Tuple(VonTuple),
    String(VonString),
    Number(VonNumber),
    Boolean(VonBoolean),
    Null(VonNull),
    Undefined(VonUndefined),
    Inf(VonInf),
    Nan(VonNan),
    Enum(VonEnum),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonTuple {
    pub elements: Vec<VonValue>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl VonValue {
    pub fn to_string(&self) -> String {
        match self {
            VonValue::Boolean(b) => b.value.to_string(),
            VonValue::Number(n) => n.value.to_string(),
            VonValue::Null(_) => "null".to_string(),
            VonValue::Undefined(_) => "undefined".to_string(),
            VonValue::Inf(_) => "inf".to_string(),
            VonValue::Nan(_) => "nan".to_string(),
            VonValue::String(s) => format!("\"{}\"", s.value),
            VonValue::Array(a) => {
                let elements: Vec<String> = a.elements.iter().map(|e| e.to_string()).collect();
                format!("[{}]", elements.join(","))
            }
            VonValue::Tuple(t) => {
                let elements: Vec<String> = t.elements.iter().map(|e| e.to_string()).collect();
                format!("({})", elements.join(","))
            }
            VonValue::Object(o) => {
                let fields: Vec<String> = o.fields.iter().map(|f| format!("{}={}", f.name, f.value.to_string())).collect();
                format!("{{{}}}", fields.join(","))
            }
            VonValue::Enum(e) => {
                if let Some(payload) = &e.payload {
                    format!("{} {}", e.variant, payload.to_string())
                }
                else {
                    e.variant.clone()
                }
            }
        }
    }
}

impl ToSource for VonValue {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            VonValue::Object(v) => v.to_source(buffer),
            VonValue::Array(v) => v.to_source(buffer),
            VonValue::Tuple(v) => v.to_source(buffer),
            VonValue::String(v) => v.to_source(buffer),
            VonValue::Number(v) => v.to_source(buffer),
            VonValue::Boolean(v) => v.to_source(buffer),
            VonValue::Null(v) => v.to_source(buffer),
            VonValue::Undefined(v) => v.to_source(buffer),
            VonValue::Inf(v) => v.to_source(buffer),
            VonValue::Nan(v) => v.to_source(buffer),
            VonValue::Enum(v) => v.to_source(buffer),
        }
    }
}

impl ToSource for VonTuple {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(");
        for (i, element) in self.elements.iter().enumerate() {
            if i > 0 {
                buffer.push(",")
            }
            element.to_source(buffer)
        }
        buffer.push(")")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonEnum {
    pub variant: String,
    pub payload: Option<Box<VonValue>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonEnum {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.variant);
        if let Some(payload) = &self.payload {
            buffer.push(" ");
            payload.to_source(buffer)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonObject {
    pub fields: Vec<VonField>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonObject {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("{");
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                buffer.push(",")
            }
            field.to_source(buffer)
        }
        buffer.push("}")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonField {
    pub name: String,
    pub value: VonValue,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonField {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.name);
        buffer.push("=");
        self.value.to_source(buffer)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonArray {
    pub elements: Vec<VonValue>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonArray {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("[");
        for (i, element) in self.elements.iter().enumerate() {
            if i > 0 {
                buffer.push(",")
            }
            element.to_source(buffer)
        }
        buffer.push("]")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonString {
    pub value: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonString {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("\"");
        buffer.push(&self.value);
        buffer.push("\"")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonNumber {
    pub value: f64,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonNumber {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonBoolean {
    pub value: bool,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonBoolean {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(if self.value { "true" } else { "false" })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonNull {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonNull {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("null")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonUndefined {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonUndefined {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("undefined")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonInf {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonInf {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("inf")
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VonNan {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for VonNan {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("nan")
    }
}
