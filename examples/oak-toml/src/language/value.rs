use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TOML value representation.
///
/// This represents the pure value of a TOML element without any source code location information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TomlValue {
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
    Array(TomlArray),
    /// Table value.
    Table(TomlTable),
}

/// Array wrapper of toml
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TomlArray {
    pub list: Vec<TomlValue>,
}

/// Table wrapper of toml
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TomlTable {
    pub dict: HashMap<String, TomlValue>,
}

impl TomlValue {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TomlValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            TomlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TomlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&Vec<TomlValue>> {
        match self {
            TomlValue::Array(TomlArray { list: a }) => Some(a),
            _ => None,
        }
    }

    /// Returns a mutable reference to the array if the value is an array.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<TomlValue>> {
        match self {
            TomlValue::Array(TomlArray { list: a }) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the table if the value is a table.
    pub fn as_table(&self) -> Option<&HashMap<String, TomlValue>> {
        match self {
            TomlValue::Table(TomlTable { dict: t }) => Some(t),
            _ => None,
        }
    }

    /// Returns a mutable reference to the table if the value is a table.
    pub fn as_table_mut(&mut self) -> Option<&mut HashMap<String, TomlValue>> {
        match self {
            TomlValue::Table(TomlTable { dict: t }) => Some(t),
            _ => None,
        }
    }

    /// Gets a value from the table by key name.
    pub fn get(&self, key: &str) -> Option<&TomlValue> {
        match self {
            TomlValue::Table(TomlTable { dict: t }) => t.get(key),
            _ => None,
        }
    }

    /// Gets a mutable value from the table by key name.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut TomlValue> {
        match self {
            TomlValue::Table(TomlTable { dict: t }) => t.get_mut(key),
            _ => None,
        }
    }
}

impl std::fmt::Display for TomlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlValue::String(s) => write!(f, "\"{}\"", s),
            TomlValue::Integer(i) => write!(f, "{}", i),
            TomlValue::Float(fl) => write!(f, "{}", fl),
            TomlValue::Boolean(b) => write!(f, "{}", b),
            TomlValue::DateTime(dt) => write!(f, "{}", dt),
            TomlValue::Array(TomlArray { list: a }) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            TomlValue::Table(TomlTable { dict: t }) => {
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
