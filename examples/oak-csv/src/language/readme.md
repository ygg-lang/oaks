# CSV Language Definition

This module contains the metadata and configuration options for the CSV language within the Oak framework.

## ⚙️ Configuration

The `CsvLanguage` struct defines how the parser and lexer should behave to accommodate various tabular data formats:

```rust
pub struct CsvLanguage {}
```

Currently, `CsvLanguage` serves as a marker struct for CSV support. Future versions may include configuration for:
- **`delimiter`**: Custom field separator (e.g., `,`, `\t`, `;`).
- **`quote_char`**: Character used for quoting fields.
- **`has_header`**: Whether the first row should be treated as a header.
- **`trim_whitespace`**: Automatically trim leading/trailing whitespace from fields.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering delimiters, quotes, and field content.
- **`ElementType`**: Defined in the `parser` module, representing records, headers, and fields.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the CSV dataset.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of CSV's tabular structure.
