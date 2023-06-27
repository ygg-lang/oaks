//! Source Map data structure and parsing.

use serde::{Deserialize, Serialize};

use crate::{Mapping, Result, SOURCE_MAP_VERSION, SourceMapError, vlq::vlq_decode_many};

/// Input source for source map parsing.
#[derive(Debug, Clone)]
pub enum SourceMapInput {
    /// JSON string input.
    Json(String),
    /// Byte slice input.
    Bytes(Vec<u8>),
    /// File path input.
    File(std::path::PathBuf),
}

impl From<String> for SourceMapInput {
    fn from(s: String) -> Self {
        SourceMapInput::Json(s)
    }
}

impl From<&str> for SourceMapInput {
    fn from(s: &str) -> Self {
        SourceMapInput::Json(s.to_string())
    }
}

impl From<Vec<u8>> for SourceMapInput {
    fn from(bytes: Vec<u8>) -> Self {
        SourceMapInput::Bytes(bytes)
    }
}

/// Source Map v3 representation.
///
/// This is the main data structure for working with source maps.
/// It follows the [Source Map v3 specification](https://sourcemaps.info/spec.html).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceMap {
    /// Version (always 3).
    pub version: u8,
    /// List of source file paths.
    #[serde(default)]
    pub sources: Vec<String>,
    /// List of source file contents (optional).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources_content: Vec<Option<String>>,
    /// List of symbol names.
    #[serde(default)]
    pub names: Vec<String>,
    /// Encoded mappings string.
    #[serde(default)]
    pub mappings: String,
    /// Output file path (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Source root (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    /// Source map references (for indexed source maps).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<SourceMapSection>,
}

/// Section in an indexed source map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapSection {
    /// Offset in the generated file.
    pub offset: SectionOffset,
    /// URL to the source map for this section.
    pub map: Option<String>,
}

/// Offset for a section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionOffset {
    /// Line offset.
    pub line: u32,
    /// Column offset.
    pub column: u32,
}

/// Metadata about a source map.
#[derive(Debug, Clone)]
pub struct SourceMapMetadata {
    /// Number of sources.
    pub sources_count: usize,
    /// Number of names.
    pub names_count: usize,
    /// Number of mappings.
    pub mappings_count: usize,
    /// Number of lines.
    pub lines_count: usize,
    /// Whether sources content is included.
    pub has_sources_content: bool,
    /// Whether it's an indexed source map.
    pub is_indexed: bool,
}

impl SourceMap {
    /// Creates a new empty source map.
    pub fn new() -> Self {
        Self { version: SOURCE_MAP_VERSION, ..Default::default() }
    }

    /// Parses a source map from JSON.
    pub fn parse(json: impl Into<SourceMapInput>) -> Result<Self> {
        let input = json.into();
        let json_str = match input {
            SourceMapInput::Json(s) => s,
            SourceMapInput::Bytes(b) => String::from_utf8(b).map_err(|e| SourceMapError::JsonError(serde_json::from_str::<serde_json::Value>(&e.to_string()).unwrap_err()))?,
            SourceMapInput::File(path) => {
                let content = std::fs::read_to_string(&path)?;
                content
            }
        };

        let mut sm: SourceMap = serde_json::from_str(&json_str)?;

        if sm.version != SOURCE_MAP_VERSION {
            return Err(SourceMapError::InvalidVersion(sm.version));
        }

        Ok(sm)
    }

    /// Converts the source map to JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(SourceMapError::from)
    }

    /// Converts the source map to pretty-printed JSON.
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(SourceMapError::from)
    }

    /// Adds a source file.
    pub fn add_source(&mut self, source: impl Into<String>) -> usize {
        let source_str = source.into();
        if let Some(idx) = self.sources.iter().position(|s| s == &source_str) {
            idx
        }
        else {
            self.sources.push(source_str);
            self.sources_content.push(None);
            self.sources.len() - 1
        }
    }

    /// Adds a name.
    pub fn add_name(&mut self, name: impl Into<String>) -> usize {
        let name_str = name.into();
        if let Some(idx) = self.names.iter().position(|n| n == &name_str) {
            idx
        }
        else {
            self.names.push(name_str);
            self.names.len() - 1
        }
    }

    /// Sets the content for a source.
    pub fn set_source_content(&mut self, index: usize, content: impl Into<String>) {
        if index < self.sources_content.len() {
            self.sources_content[index] = Some(content.into());
        }
    }

    /// Gets the source path at an index.
    pub fn get_source(&self, index: usize) -> Option<&str> {
        self.sources.get(index).map(|s| s.as_str())
    }

    /// Gets the name at an index.
    pub fn get_name(&self, index: usize) -> Option<&str> {
        self.names.get(index).map(|s| s.as_str())
    }

    /// Gets the source content at an index.
    pub fn get_source_content(&self, index: usize) -> Option<Option<&String>> {
        self.sources_content.get(index).map(|c| c.as_ref())
    }

    /// Parses all mappings into a vector.
    pub fn parse_mappings(&self) -> Result<Vec<Mapping>> {
        let mut mappings = Vec::new();
        let mut generated_line = 0u32;
        let mut generated_column = 0u32;
        let mut source_index = 0u32;
        let mut original_line = 0u32;
        let mut original_column = 0u32;
        let mut name_index = 0u32;

        for line in self.mappings.split(';') {
            if line.is_empty() {
                generated_line += 1;
                generated_column = 0;
                continue;
            }

            for segment in line.split(',') {
                if segment.is_empty() {
                    continue;
                }

                let values = vlq_decode_many(segment)?;

                generated_column = (generated_column as i32 + values.get(0).copied().unwrap_or(0)) as u32;

                let mut mapping = Mapping::generated_only(generated_line, generated_column);

                if values.len() >= 5 {
                    source_index = (source_index as i32 + values[1]) as u32;
                    original_line = (original_line as i32 + values[2]) as u32;
                    original_column = (original_column as i32 + values[3]) as u32;

                    mapping.source_index = Some(source_index);
                    mapping.original_line = Some(original_line);
                    mapping.original_column = Some(original_column);

                    if values.len() >= 6 {
                        name_index = (name_index as i32 + values[5]) as u32;
                        mapping.name_index = Some(name_index);
                    }
                }

                mappings.push(mapping);
            }

            generated_line += 1;
            generated_column = 0;
        }

        Ok(mappings)
    }

    /// Returns metadata about this source map.
    pub fn metadata(&self) -> SourceMapMetadata {
        let mappings = self.parse_mappings().ok();
        let lines_count = self.mappings.split(';').count();

        SourceMapMetadata {
            sources_count: self.sources.len(),
            names_count: self.names.len(),
            mappings_count: mappings.map(|m| m.len()).unwrap_or(0),
            lines_count,
            has_sources_content: self.sources_content.iter().any(|c| c.is_some()),
            is_indexed: !self.sections.is_empty(),
        }
    }

    /// Generates the inline source map comment.
    pub fn to_inline_comment(&self) -> Result<String> {
        use base64::prelude::*;
        let json = self.to_json()?;
        let encoded = BASE64_STANDARD.encode(json.as_bytes());
        Ok(format!("//# sourceMappingURL=data:application/json;base64,{}", encoded))
    }

    /// Generates the external source map comment.
    pub fn to_external_comment(&self, filename: &str) -> String {
        format!("//# sourceMappingURL={}", filename)
    }

    /// Checks if this is an indexed source map.
    pub fn is_indexed(&self) -> bool {
        !self.sections.is_empty()
    }

    /// Checks if this source map has sources content.
    pub fn has_sources_content(&self) -> bool {
        self.sources_content.iter().any(|c| c.is_some())
    }

    /// Returns the full source path for a source index.
    pub fn get_full_source_path(&self, index: usize) -> Option<String> {
        self.sources.get(index).map(|source| if let Some(ref root) = self.source_root { format!("{}{}", root, source) } else { source.clone() })
    }
}

impl Default for SourceMapMetadata {
    fn default() -> Self {
        Self { sources_count: 0, names_count: 0, mappings_count: 0, lines_count: 0, has_sources_content: false, is_indexed: false }
    }
}
