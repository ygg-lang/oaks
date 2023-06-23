# ActionScript Builder

The `ActionScriptBuilder` is responsible for bridging the gap between the low-level, lossless syntax trees (Green/Red Trees) and the high-level, strongly-typed AST.

## 🏗️ Responsibilities

- **Tree Transformation**: Converts the `GreenNode` produced by the parser into a `RedNode` hierarchy for easier navigation.
- **Typed Mapping**: Maps the untyped `RedNode` elements into the structured `ActionScriptRoot` and `ActionScriptItem` types defined in the `ast` module.
- **Diagnostics Integration**: Aggregates diagnostics from the lexing and parsing phases and potentially adds builder-specific validation errors.

## 🔄 Integration with Oak

As a core part of the Oak ecosystem, the builder implements the `Builder` trait, allowing it to participate in the incremental compilation pipeline. It ensures that any changes to the source code are reflected in an updated AST with minimal re-computation.
