//! Source Map Composer for combining multiple source maps.
//!
//! This is useful when you have a pipeline of transformations,
//! e.g., TypeScript -> JavaScript -> Minified JavaScript.

use crate::{Mapping, Result, SourceMap, SourceMapBuilder, SourceMapError};

/// Composer for combining multiple source maps.
///
/// When you have a chain of transformations, you can compose
/// the source maps to get a direct mapping from the original
/// source to the final output.
#[derive(Debug, Default)]
pub struct SourceMapComposer {
    maps: Vec<SourceMap>,
}

impl SourceMapComposer {
    /// Creates a new empty composer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a source map to the composition chain.
    ///
    /// Maps should be added in the order of transformations.
    /// For example, if you have:
    /// - TypeScript -> JavaScript (map1)
    /// - JavaScript -> Minified (map2)
    /// You should add map1 first, then map2.
    pub fn add(mut self, map: SourceMap) -> Self {
        self.maps.push(map);
        self
    }

    /// Composes all added source maps into one.
    pub fn compose(self) -> Result<SourceMap> {
        if self.maps.is_empty() {
            return Ok(SourceMap::new());
        }

        if self.maps.len() == 1 {
            return Ok(self.maps.into_iter().next().unwrap());
        }

        let mut maps = self.maps.into_iter();
        let first = maps.next().unwrap();

        let result = maps.try_fold(first, |acc, map| compose_two(&acc, &map))?;

        Ok(result)
    }
}

/// Composes two source maps.
///
/// The first map goes from generated to intermediate,
/// the second map goes from intermediate to original.
/// The result goes from generated to original.
pub fn compose_two(map1: &SourceMap, map2: &SourceMap) -> Result<SourceMap> {
    let mut builder = SourceMapBuilder::new();

    let mappings1 = map1.parse_mappings()?;

    for mapping in mappings1 {
        if let (Some(source_idx), Some(orig_line), Some(orig_col)) = (mapping.source_index, mapping.original_line, mapping.original_column) {
            let intermediate_source = map1.get_source(source_idx as usize).ok_or_else(|| SourceMapError::InvalidSourceIndex(source_idx as usize))?;

            let intermediate_idx = map2.sources.iter().position(|s| s == intermediate_source);

            if let Some(idx) = intermediate_idx {
                let decoder = crate::SourceMapDecoder::new(map2.clone())?;

                if let Some(intermediate_mapping) = decoder.lookup(orig_line, orig_col) {
                    if let (Some(final_source_idx), Some(final_line), Some(final_col)) = (intermediate_mapping.source_index, intermediate_mapping.original_line, intermediate_mapping.original_column) {
                        let new_source_idx = builder.add_source(map2.get_source(final_source_idx as usize).unwrap_or(""));

                        builder.add_mapping(mapping.generated_line, mapping.generated_column, Some(new_source_idx), Some(final_line), Some(final_col), intermediate_mapping.name_index.or(mapping.name_index));
                        continue;
                    }
                }
            }
        }

        builder.add_mapping(mapping.generated_line, mapping.generated_column, None, None, None, None);
    }

    Ok(builder.build())
}
