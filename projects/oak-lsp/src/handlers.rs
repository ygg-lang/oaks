use core::range::Range;
use serde::Deserialize;

/// Request for a resource by URI.
#[derive(Deserialize)]
pub struct UriRequest {
    /// The URI of the resource.
    pub uri: String,
}

/// Request for a resource within a specific byte range.
#[derive(Deserialize)]
pub struct RangeRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The byte range within the resource.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

/// Request for a resource at a specific byte position.
#[derive(Deserialize)]
pub struct PositionRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The byte position within the resource.
    pub position: usize,
}

/// Request for searching resources with a query string.
#[derive(Deserialize)]
pub struct QueryRequest {
    /// The search query.
    pub query: String,
}

/// Request for renaming a symbol in a resource.
#[derive(Deserialize)]
pub struct RenameRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The range of the symbol to be renamed.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    /// The new name for the symbol.
    pub new_name: String,
}
