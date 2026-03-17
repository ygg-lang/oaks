#![doc = include_str!("readme.md")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod builder;
mod composer;
mod decoder;
mod error;
mod mapping;
mod source_map;
mod vlq;

pub use builder::SourceMapBuilder;
pub use composer::SourceMapComposer;
pub use decoder::SourceMapDecoder;
pub use error::{Result, SourceMapError};
pub use mapping::{BoundedMapping, Mapping, Segment};
pub use source_map::{SourceMap, SourceMapMetadata};
pub use vlq::{vlq_decode, vlq_decode_many, vlq_encode, vlq_encode_many};

pub use source_map::SourceMapInput;

/// Source Map version (always 3).
pub const SOURCE_MAP_VERSION: u8 = 3;

/// The default source root.
pub const DEFAULT_SOURCE_ROOT: &str = "";
