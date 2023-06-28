use std::collections::HashMap;

/// TOML value representation.
///
/// This represents the pure value of a TOML element without any source code location information.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String value.
    String(String),
    /// Integer value.
    Integer(i64),
    /// Floating-point value.
    Float(f64),
    /// Boolean value.
    Boolean(bool),
    /// Date-time value as a string.
    DateTime(String),
    /// Array value.
    Array(Vec<Value>),
    /// Table value.
    Table(HashMap<String, Value>),
}

impl Value {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the table if the value is a table.
    pub fn as_table(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Gets a value from the table by key name.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Table(t) => t.get(key),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::DateTime(dt) => write!(f, "{}", dt),
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Table(t) => {
                write!(f, "{{")?;
                for (i, (key, value)) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}
