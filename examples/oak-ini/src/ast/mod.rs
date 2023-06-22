use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IniRoot {
    pub sections: Vec<Section>,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub properties: Vec<Property>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Property {
    pub key: String,
    pub value: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
