# Red-Green Tree

Red-green tree implementation for efficient kind tree representation.

This module provides the core red-green tree data structures that enable
efficient incremental parsing and kind tree manipulation.

## Key Components

- **Green Trees**: Immutable, position-agnostic kind tree nodes allocated in an Arena.
- **Red Trees**: Position-aware kind tree nodes computed from green trees.

## Architecture

The red-green tree design enables:
- **Incremental Parsing**: Only re-parse changed regions of source code
- **Memory Efficiency**: Arena-based allocation with minimal overhead
- **Performance**: Zero-copy node construction and fast traversal
