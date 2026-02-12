//! Serde support for `core::range::Range`.

#[cfg(feature = "serde")]
use core::range::Range;

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct RangeDef<T> {
    start: T,
    end: T,
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize)]
struct RangeDefRef<'a, T> {
    start: &'a T,
    end: &'a T,
}

/// Serializes a `Range<T>`.
#[cfg(feature = "serde")]
pub fn serialize<S, T>(value: &Range<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize,
{
    serde::Serialize::serialize(&RangeDefRef { start: &value.start, end: &value.end }, serializer)
}

/// Deserializes a `Range<T>`.
#[cfg(feature = "serde")]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Range<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let def = <RangeDef<T> as serde::Deserialize>::deserialize(deserializer)?;
    Ok(Range { start: def.start, end: def.end })
}
