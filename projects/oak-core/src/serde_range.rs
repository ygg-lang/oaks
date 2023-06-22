//! Serde support for `core::range::Range`.

use core::range::Range;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
struct RangeDef<T> {
    start: T,
    end: T,
}

#[derive(Serialize)]
struct RangeDefRef<'a, T> {
    start: &'a T,
    end: &'a T,
}

/// Serializes a `Range<T>`.
pub fn serialize<S, T>(value: &Range<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    RangeDefRef { start: &value.start, end: &value.end }.serialize(serializer)
}

/// Deserializes a `Range<T>`.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Range<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let def = RangeDef::<T>::deserialize(deserializer)?;
    Ok(Range { start: def.start, end: def.end })
}

/// Serde support for `Option<Range<T>>`.
pub mod option {
    use super::{RangeDef, RangeDefRef};
    use core::range::Range;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serializes an `Option<Range<T>>`.
    pub fn serialize<S, T>(value: &Option<Range<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match value {
            Some(r) => Some(RangeDefRef { start: &r.start, end: &r.end }).serialize(serializer),
            None => Option::<RangeDef<T>>::None.serialize(serializer),
        }
    }

    /// Deserializes an `Option<Range<T>>`.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Range<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let def = Option::<RangeDef<T>>::deserialize(deserializer)?;
        Ok(def.map(|d| Range { start: d.start, end: d.end }))
    }
}
