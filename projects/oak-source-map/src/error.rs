//! Error types for source map operations.

/// Result type alias for source map operations.
pub type Result<T> = std::result::Result<T, SourceMapError>;

/// Error type for source map operations.
#[derive(Debug)]
pub enum SourceMapError {
    /// Invalid source map version.
    InvalidVersion(u8),

    /// Missing required field.
    MissingField(&'static str),

    /// Invalid VLQ encoding.
    InvalidVlq {
        /// Position in the mappings string.
        position: usize,
        /// Error message.
        message: String,
    },

    /// Invalid mapping.
    InvalidMapping {
        /// Line number.
        line: u32,
        /// Column number.
        column: u32,
        /// Error message.
        message: String,
    },

    /// JSON parsing error.
    JsonError(serde_json::Error),

    /// IO error.
    IoError(std::io::Error),

    /// Index out of bounds.
    IndexOutOfBounds {
        /// The index that was out of bounds.
        index: usize,
        /// The length of the collection.
        length: usize,
    },

    /// Invalid source index.
    InvalidSourceIndex(usize),

    /// Invalid name index.
    InvalidNameIndex(usize),

    /// Source map composition error.
    CompositionError(String),
}

impl PartialEq for SourceMapError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SourceMapError::InvalidVersion(a), SourceMapError::InvalidVersion(b)) => a == b,
            (SourceMapError::MissingField(a), SourceMapError::MissingField(b)) => a == b,
            (SourceMapError::InvalidVlq { position: pos_a, message: msg_a }, SourceMapError::InvalidVlq { position: pos_b, message: msg_b }) => pos_a == pos_b && msg_a == msg_b,
            (SourceMapError::InvalidMapping { line: line_a, column: col_a, message: msg_a }, SourceMapError::InvalidMapping { line: line_b, column: col_b, message: msg_b }) => line_a == line_b && col_a == col_b && msg_a == msg_b,
            (SourceMapError::IndexOutOfBounds { index: idx_a, length: len_a }, SourceMapError::IndexOutOfBounds { index: idx_b, length: len_b }) => idx_a == idx_b && len_a == len_b,
            (SourceMapError::InvalidSourceIndex(a), SourceMapError::InvalidSourceIndex(b)) => a == b,
            (SourceMapError::InvalidNameIndex(a), SourceMapError::InvalidNameIndex(b)) => a == b,
            (SourceMapError::CompositionError(a), SourceMapError::CompositionError(b)) => a == b,
            (SourceMapError::JsonError(_), SourceMapError::JsonError(_)) => true,
            (SourceMapError::IoError(_), SourceMapError::IoError(_)) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for SourceMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceMapError::InvalidVersion(version) => {
                write!(f, "Invalid source map version: expected 3, got {}", version)
            }
            SourceMapError::MissingField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            SourceMapError::InvalidVlq { position, message } => {
                write!(f, "Invalid VLQ encoding at position {}: {}", position, message)
            }
            SourceMapError::InvalidMapping { line, column, message } => {
                write!(f, "Invalid mapping at line {}, column {}: {}", line, column, message)
            }
            SourceMapError::JsonError(err) => {
                write!(f, "JSON parsing error: {}", err)
            }
            SourceMapError::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
            SourceMapError::IndexOutOfBounds { index, length } => {
                write!(f, "Index out of bounds: {} >= {}", index, length)
            }
            SourceMapError::InvalidSourceIndex(index) => {
                write!(f, "Invalid source index: {}", index)
            }
            SourceMapError::InvalidNameIndex(index) => {
                write!(f, "Invalid name index: {}", index)
            }
            SourceMapError::CompositionError(message) => {
                write!(f, "Source map composition error: {}", message)
            }
        }
    }
}

impl std::error::Error for SourceMapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SourceMapError::JsonError(err) => Some(err),
            SourceMapError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for SourceMapError {
    fn from(err: serde_json::Error) -> Self {
        SourceMapError::JsonError(err)
    }
}

impl From<std::io::Error> for SourceMapError {
    fn from(err: std::io::Error) -> Self {
        SourceMapError::IoError(err)
    }
}

impl SourceMapError {
    /// Creates a new invalid VLQ error.
    pub fn invalid_vlq(position: usize, message: impl Into<String>) -> Self {
        SourceMapError::InvalidVlq { position, message: message.into() }
    }

    /// Creates a new invalid mapping error.
    pub fn invalid_mapping(line: u32, column: u32, message: impl Into<String>) -> Self {
        SourceMapError::InvalidMapping { line, column, message: message.into() }
    }

    /// Creates a new index out of bounds error.
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        SourceMapError::IndexOutOfBounds { index, length }
    }
}
