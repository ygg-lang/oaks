use core::range::Range;

/// Request for a resource by URI.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UriRequest {
    /// The URI of the resource.
    pub uri: String,
}

/// Request for a resource within a specific byte range.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RangeRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The byte range within the resource.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// Request for a resource at a specific byte position.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PositionRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The byte position within the resource.
    pub position: usize,
}

/// Request for searching resources with a query string.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct QueryRequest {
    /// The search query.
    pub query: String,
}

/// Request for renaming a symbol in a resource.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RenameRequest {
    /// The URI of the resource.
    pub uri: String,
    /// The range of the symbol to be renamed.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
    /// The new name for the symbol.
    pub new_name: String,
}
