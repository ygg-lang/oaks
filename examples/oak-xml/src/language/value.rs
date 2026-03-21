use serde::Deserialize;
use std::collections::HashMap;

/// XML value representation.
///
/// This represents the pure value of an XML element without any source code location information.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum XmlValue {
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
    Array(XmlArray),
    /// Object value.
    Object(XmlObject),
}

/// Array wrapper of XML
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct XmlArray {
    pub list: Vec<XmlValue>,
}

/// Object wrapper of XML
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct XmlObject {
    pub dict: HashMap<String, XmlValue>,
}

impl XmlValue {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            XmlValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            XmlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the floating-point value if the value is a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            XmlValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            XmlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns true if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, XmlValue::Null)
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&Vec<XmlValue>> {
        match self {
            XmlValue::Array(XmlArray { list: a }) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the object if the value is an object.
    pub fn as_object(&self) -> Option<&HashMap<String, XmlValue>> {
        match self {
            XmlValue::Object(XmlObject { dict: o }) => Some(o),
            _ => None,
        }
    }

    /// Gets a value from the object by key name.
    pub fn get(&self, key: &str) -> Option<&XmlValue> {
        match self {
            XmlValue::Object(XmlObject { dict: o }) => o.get(key),
            _ => None,
        }
    }
}

impl std::fmt::Display for XmlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XmlValue::String(s) => write!(f, "\"{}\"", s),
            XmlValue::Integer(i) => write!(f, "{}", i),
            XmlValue::Float(fl) => write!(f, "{}", fl),
            XmlValue::Boolean(b) => write!(f, "{}", b),
            XmlValue::Null => write!(f, "null"),
            XmlValue::Array(XmlArray { list: a }) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            XmlValue::Object(XmlObject { dict: o }) => {
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
