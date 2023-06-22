use core::range::Range;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UriRequest {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct RangeRequest {
    pub uri: String,
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

#[derive(Deserialize)]
pub struct PositionRequest {
    pub uri: String,
    pub position: usize,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Deserialize)]
pub struct RenameRequest {
    pub uri: String,
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    pub new_name: String,
}
