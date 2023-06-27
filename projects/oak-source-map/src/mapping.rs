//! Mapping types for Source Maps.

use serde::{Deserialize, Serialize};

/// A single mapping entry in a source map.
///
/// Each mapping describes how a generated position maps back to
/// an original position in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mapping {
    /// Generated line (0-indexed).
    pub generated_line: u32,
    /// Generated column (0-indexed).
    pub generated_column: u32,
    /// Source file index (if this mapping has a source).
    pub source_index: Option<u32>,
    /// Original line (0-indexed, if this mapping has a source).
    pub original_line: Option<u32>,
    /// Original column (0-indexed, if this mapping has a source).
    pub original_column: Option<u32>,
    /// Name index (if this mapping has a name).
    pub name_index: Option<u32>,
}

impl Mapping {
    /// Creates a new mapping with only generated position.
    pub fn generated_only(line: u32, column: u32) -> Self {
        Self { generated_line: line, generated_column: column, source_index: None, original_line: None, original_column: None, name_index: None }
    }

    /// Creates a new mapping with full information.
    pub fn full(generated_line: u32, generated_column: u32, source_index: u32, original_line: u32, original_column: u32, name_index: Option<u32>) -> Self {
        Self { generated_line, generated_column, source_index: Some(source_index), original_line: Some(original_line), original_column: Some(original_column), name_index }
    }

    /// Checks if this mapping has source information.
    pub fn has_source(&self) -> bool {
        self.source_index.is_some()
    }

    /// Checks if this mapping has a name.
    pub fn has_name(&self) -> bool {
        self.name_index.is_some()
    }

    /// Returns the source index, or 0 if none.
    pub fn source_index_or_zero(&self) -> u32 {
        self.source_index.unwrap_or(0)
    }

    /// Returns the original line, or 0 if none.
    pub fn original_line_or_zero(&self) -> u32 {
        self.original_line.unwrap_or(0)
    }

    /// Returns the original column, or 0 if none.
    pub fn original_column_or_zero(&self) -> u32 {
        self.original_column.unwrap_or(0)
    }

    /// Returns the name index, or 0 if none.
    pub fn name_index_or_zero(&self) -> u32 {
        self.name_index.unwrap_or(0)
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.generated_line.cmp(&other.generated_line).then_with(|| self.generated_column.cmp(&other.generated_column))
    }
}

/// A segment in the mappings string.
///
/// Segments are separated by commas within a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    /// Generated column (relative to previous segment on same line).
    pub generated_column: i32,
    /// Source index (relative to previous segment, if present).
    pub source_index: Option<i32>,
    /// Original line (relative to previous segment, if present).
    pub original_line: Option<i32>,
    /// Original column (relative to previous segment, if present).
    pub original_column: Option<i32>,
    /// Name index (relative to previous segment, if present).
    pub name_index: Option<i32>,
}

impl Segment {
    /// Creates a new segment with only generated column.
    pub fn generated_only(column: i32) -> Self {
        Self { generated_column: column, source_index: None, original_line: None, original_column: None, name_index: None }
    }

    /// Creates a new full segment.
    pub fn full(generated_column: i32, source_index: i32, original_line: i32, original_column: i32, name_index: Option<i32>) -> Self {
        Self { generated_column, source_index: Some(source_index), original_line: Some(original_line), original_column: Some(original_column), name_index }
    }

    /// Checks if this segment has source information.
    pub fn has_source(&self) -> bool {
        self.source_index.is_some()
    }

    /// Checks if this segment has a name.
    pub fn has_name(&self) -> bool {
        self.name_index.is_some()
    }
}

/// A mapping with bounds information for efficient lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedMapping {
    /// The mapping.
    pub mapping: Mapping,
    /// The start column of the generated range.
    pub start_column: u32,
    /// The end column of the generated range (exclusive).
    pub end_column: u32,
}

impl BoundedMapping {
    /// Creates a new bounded mapping.
    pub fn new(mapping: Mapping, start_column: u32, end_column: u32) -> Self {
        Self { mapping, start_column, end_column }
    }

    /// Checks if a column is within this mapping's range.
    pub fn contains_column(&self, column: u32) -> bool {
        column >= self.start_column && column < self.end_column
    }
}
