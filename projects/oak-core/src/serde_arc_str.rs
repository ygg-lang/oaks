//! Serde support for `triomphe::Arc<str>`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use triomphe::Arc;

/// Serializes an `Arc<str>`.
#[cfg(feature = "serde")]
pub fn serialize<S>(value: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

/// Deserializes an `Arc<str>`.
#[cfg(feature = "serde")]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Arc::from(s))
}
