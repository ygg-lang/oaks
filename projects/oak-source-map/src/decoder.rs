//! Source Map Decoder for efficient lookup.

use std::collections::BTreeMap;

use crate::{BoundedMapping, Mapping, Result, SourceMap, SourceMapError};

/// Decoder for efficient source map lookups.
///
/// This provides O(log n) lookup for original positions from generated positions.
#[derive(Debug, Clone)]
pub struct SourceMapDecoder {
    source_map: SourceMap,
    lines: BTreeMap<u32, Vec<BoundedMapping>>,
    sources: Vec<String>,
    names: Vec<String>,
}

impl SourceMapDecoder {
    /// Creates a new decoder from a source map.
    pub fn new(source_map: SourceMap) -> Result<Self> {
        let mappings = source_map.parse_mappings()?;
        let mut lines: BTreeMap<u32, Vec<BoundedMapping>> = BTreeMap::new();

        for mapping in mappings {
            let line = mapping.generated_line;
            let col = mapping.generated_column;

            let line_mappings = lines.entry(line).or_default();

            if let Some(last) = line_mappings.last_mut() {
                last.end_column = col;
            }

            line_mappings.push(BoundedMapping::new(mapping, col, u32::MAX));
        }

        Ok(Self { source_map, lines, sources: Vec::new(), names: Vec::new() })
    }

    /// Looks up the original position for a generated position.
    pub fn lookup(&self, generated_line: u32, generated_column: u32) -> Option<&Mapping> {
        let line_mappings = self.lines.get(&generated_line)?;

        let idx = line_mappings
            .binary_search_by(|m| {
                if m.end_column <= generated_column {
                    std::cmp::Ordering::Less
                }
                else if m.start_column > generated_column {
                    std::cmp::Ordering::Greater
                }
                else {
                    std::cmp::Ordering::Equal
                }
            })
            .ok()?;

        Some(&line_mappings[idx].mapping)
    }

    /// Looks up the original position and returns full information.
    pub fn lookup_full(&self, generated_line: u32, generated_column: u32) -> Option<OriginalPosition> {
        let mapping = self.lookup(generated_line, generated_column)?;

        let source = mapping.source_index.and_then(|idx| self.source_map.get_source(idx as usize));

        let name = mapping.name_index.and_then(|idx| self.source_map.get_name(idx as usize));

        Some(OriginalPosition { source: source.map(String::from), original_line: mapping.original_line, original_column: mapping.original_column, name: name.map(String::from) })
    }

    /// Returns all mappings for a generated line.
    pub fn get_line_mappings(&self, line: u32) -> Option<&[BoundedMapping]> {
        self.lines.get(&line).map(|v| v.as_slice())
    }

    /// Returns the number of lines in the generated file.
    pub fn generated_line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns the underlying source map.
    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    /// Iterates over all mappings.
    pub fn iter_mappings(&self) -> impl Iterator<Item = &BoundedMapping> {
        self.lines.values().flat_map(|v| v.iter())
    }
}

/// Original position information from a lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalPosition {
    /// Source file path.
    pub source: Option<String>,
    /// Original line (0-indexed).
    pub original_line: Option<u32>,
    /// Original column (0-indexed).
    pub original_column: Option<u32>,
    /// Symbol name.
    pub name: Option<String>,
}

impl OriginalPosition {
    /// Creates a new original position.
    pub fn new(source: Option<String>, original_line: Option<u32>, original_column: Option<u32>, name: Option<String>) -> Self {
        Self { source, original_line, original_column, name }
    }

    /// Checks if this position has source information.
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    /// Checks if this position has a name.
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
}
