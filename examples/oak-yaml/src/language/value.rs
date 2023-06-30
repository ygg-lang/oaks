use serde::Deserialize;
use std::collections::HashMap;

/// YAML value representation.
///
/// This represents the pure value of a YAML element without any source code location information.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum YamlValue {
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
    Array(YamlArray),
    /// Object value.
    Object(YamlObject),
}

/// Array wrapper of YAML
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YamlArray {
    pub list: Vec<YamlValue>,
}

/// Object wrapper of YAML
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YamlObject {
    pub dict: HashMap<String, YamlValue>,
}

impl YamlValue {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            YamlValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            YamlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the floating-point value if the value is a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            YamlValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            YamlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns true if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, YamlValue::Null)
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&Vec<YamlValue>> {
        match self {
            YamlValue::Array(YamlArray { list: a }) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the object if the value is an object.
    pub fn as_object(&self) -> Option<&HashMap<String, YamlValue>> {
        match self {
            YamlValue::Object(YamlObject { dict: o }) => Some(o),
            _ => None,
        }
    }

    /// Gets a value from the object by key name.
    pub fn get(&self, key: &str) -> Option<&YamlValue> {
        match self {
            YamlValue::Object(YamlObject { dict: o }) => o.get(key),
            _ => None,
        }
    }
}

impl std::fmt::Display for YamlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamlValue::String(s) => write!(f, "\"{}\"", s),
            YamlValue::Integer(i) => write!(f, "{}", i),
            YamlValue::Float(fl) => write!(f, "{}", fl),
            YamlValue::Boolean(b) => write!(f, "{}", b),
            YamlValue::Null => write!(f, "null"),
            YamlValue::Array(YamlArray { list: a }) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            YamlValue::Object(YamlObject { dict: o }) => {
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
