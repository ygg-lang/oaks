//! Serde support for `triomphe::Arc<str>`.

#[cfg(feature = "serde")]
use std::collections::BTreeMap;
#[cfg(feature = "serde")]
use triomphe::Arc;

/// Serializes an `Arc<str>`.
#[cfg(feature = "serde")]
pub fn serialize<S>(value: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(value)
}

/// Deserializes an `Arc<str>`.
#[cfg(feature = "serde")]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <String as serde::Deserialize>::deserialize(deserializer)?;
    Ok(Arc::from(s))
}

/// Serde support for `BTreeMap<Arc<str>, V>`.
#[cfg(feature = "serde")]
pub mod map {
    use super::*;

    /// Serializes a `BTreeMap<Arc<str>, V>`.
    pub fn serialize<S, V>(map: &BTreeMap<Arc<str>, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        V: serde::Serialize,
    {
        let mut map_ser = serializer.serialize_map(Some(map.len()))?;
        for (key, value) in map {
            serde::ser::SerializeMap::serialize_entry(&mut map_ser, &**key, value)?;
        }
        serde::ser::SerializeMap::end(map_ser)
    }

    /// Deserializes a `BTreeMap<Arc<str>, V>`.
    pub fn deserialize<'de, D, V>(deserializer: D) -> Result<BTreeMap<Arc<str>, V>, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: serde::Deserialize<'de>,
    {
        struct MapVisitor<V>(std::marker::PhantomData<V>);

        impl<'de, V> serde::de::Visitor<'de> for MapVisitor<V>
        where
            V: serde::Deserialize<'de>,
        {
            type Value = BTreeMap<Arc<str>, V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut map = BTreeMap::new();
                while let Some((key, value)) = access.next_entry::<String, V>()? {
                    map.insert(Arc::from(key), value);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(MapVisitor(std::marker::PhantomData))
    }
}
