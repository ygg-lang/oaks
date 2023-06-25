# DSV Language Definition

This module contains the metadata and configuration options for the DSV (Delimiter-Separated Values) language within the Oak framework.

## Overview

The `DsvLanguage` struct defines how the parser and lexer should behave to accommodate various tabular data formats:

```rust
pub struct DsvLanguage {
    pub field_separator: char,
    pub quote_char: char,
}
```

Currently, `DsvLanguage` provides configuration for:
- **`field_separator`**: The character used to separate fields (e.g., `,` for CSV, `\t` for TSV).
- **`quote_char`**: The character used for quoting fields (e.g., `"`).

## Usage

The `DsvLanguage` configuration is used throughout the `oak-dsv` crate via const generics:

- **`Dsv<const LANG: DsvLanguage>`**: A marker struct for the language.
- **`DsvRoot<const LANG: DsvLanguage>`**: Defined in the `ast` module, providing a strongly-typed view of the DSV dataset.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of DSV's tabular structure.
