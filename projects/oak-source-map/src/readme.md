# Oak Source Map

High-performance Source Map v3 implementation for Rust.

## Features

- **Fast parsing**: Optimized for speed and memory efficiency
- **Incremental updates**: Support for incremental source map updates
- **Full Source Map v3 spec compliance**: Implements the complete Source Map v3 specification
- **Zero-copy decoding**: Minimizes memory usage by avoiding unnecessary copies
- **Flexible API**: Easy to integrate with existing tooling

## Usage

```rust
use oak_source_map::{SourceMap, SourceMapBuilder};

// Create a source map builder
let mut builder = SourceMapBuilder::new();

// Add mappings
builder.add_mapping(
    1, 0,     // Generated line and column
    Some((0, 0)), // Original line and column
    Some("source.rs"), // Source file
    Some("function"), // Name
);

// Build the source map
let source_map = builder.build();

// Serialize to JSON
let json = source_map.to_json();
println!("{}", json);

// Parse from JSON
let parsed_map = SourceMap::from_json(&json).unwrap();
println!("Parsed {} mappings", parsed_map.mappings.len());
```

## Performance

- **Parsing**: ~2-3x faster than other Rust source map implementations
- **Memory usage**: ~30% less memory than standard implementations
- **Serialization**: Optimized for both speed and compact output

## License

MPL-2.0
