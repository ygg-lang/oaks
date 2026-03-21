use serde::Deserialize;
use std::collections::HashMap;

/// JSON value representation.
///
/// This represents the pure value of a JSON element without any source code location information.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum JsonValue {
    /// String value.
    String(String),
    /// Integer value.
    Integer(i64),
    /// Floating-point value.
    Float(f64),
    /// Boolean value.
    Boolean(bool),
    /// Null value.
    Null,
    /// Array value.
    Array(JsonArray),
    /// Object value.
    Object(JsonObject),
}

/// Array wrapper of JSON
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JsonArray {
    pub list: Vec<JsonValue>,
}

/// Object wrapper of JSON
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JsonObject {
    pub dict: HashMap<String, JsonValue>,
}

impl JsonValue {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            JsonValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the floating-point value if the value is a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            JsonValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns true if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(JsonArray { list: a }) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the object if the value is an object.
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(JsonObject { dict: o }) => Some(o),
            _ => None,
        }
    }

    /// Gets a value from the object by key name.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(JsonObject { dict: o }) => o.get(key),
            _ => None,
        }
    }
}

impl std::fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Integer(i) => write!(f, "{}", i),
            JsonValue::Float(fl) => write!(f, "{}", fl),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Array(JsonArray { list: a }) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(JsonObject { dict: o }) => {
                write!(f, "{{")?;
                for (i, (key, value)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}
