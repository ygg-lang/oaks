# oak-highlight

A lightweight and customizable syntax highlighter for the Oak framework.

## Features

- **Multi-language Support**: Designed to work with any language defined in the Oak framework.
- **Customizable Themes**: Supports multiple themes including One Dark Pro and One Light.
- **Extensible Exporters**: Export highlighted code to HTML, CSS, JSON, and ANSI (terminal).
- **Universal Element Roles**: Uses Oak's universal element roles for consistent highlighting across different languages.

## Usage

```rust
use oak_highlight::{OakHighlighter, Theme, HtmlExporter, Exporter};

// Create a highlighter with a theme
let highlighter = OakHighlighter::new().theme(Theme::OneDarkPro);

// Highlight source code
let source = "fn main() { println!(\"Hello, world!\"); }";
let result = highlighter.highlight(source, "rust", Theme::OneDarkPro).unwrap();

// Export to HTML
let exporter = HtmlExporter::new(true, true);
let html = exporter.export(&result);
```

## Supported Formats

- **HTML**: Generates HTML with inline styles or CSS classes.
- **CSS**: Generates CSS style definitions.
- **JSON**: Generates a JSON representation of the highlighted segments.
- **ANSI**: Generates ANSI escape codes for terminal output.
