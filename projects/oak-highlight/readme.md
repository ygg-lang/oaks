# Oak Syntax Highlighter

[![Crates.io](https://img.shields.io/crates/v/oak-highlight.svg)](https://crates.io/crates/oak-highlight)
[![Documentation](https://docs.rs/oak-highlight/badge.svg)](https://docs.rs/oak-highlight)

A powerful syntax highlighter supporting multiple programming languages, built on oak-core for accurate tokenization and beautiful code presentation.

## 🎯 Overview

Oak of highlight is a comprehensive syntax highlighter designed to provide beautiful and accurate code highlighting for multiple programming languages. Built on the solid foundation of oak-core, it offers detailed tokenization, customizable themes, and efficient rendering for various output formats.

## ✨ Features

- **Multi-Language Support**: Highlight code in 100+ programming languages
- **Customizable Themes**: Built-in themes with custom theme support
- **Accurate Tokenization**: Precise lexical analysis using oak-core parsers
- **Multiple Output Formats**: HTML, ANSI terminal colors, LaTeX
- **Performance Optimized**: Fast highlighting with minimal allocations
- **Language Detection**: Automatic language detection from file extensions

## 🚀 Quick Start

Basic example:

```rust
use oak_highlight::{OakHighlighter, Theme, Highlighter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let highlighter = OakHighlighter::new();
    let code = r#"fn main() {
    println!("Hello, World!");
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("Number: {}", n);
    }
}"#;
    
    let highlighted = highlighter.highlight(code, "rust", Theme::GitHub)?;
    println!("Highlighted code:\n{:?}", highlighted);
    Ok(())
}
```

## 📋 Highlighting Examples

### Rust Code
```rust
use oak_highlight::{OakHighlighter, Theme, Highlighter};

let highlighter = OakHighlighter::new();
let rust_code = r#"use std::collections::HashMap;

fn process_data(items: Vec<&str>) -> Result<HashMap<String, usize>, Error> {
    let mut counts = HashMap::new();
    
    for item in items {
        *counts.entry(item.to_string()).or_insert(0) += 1;
    }
    
    Ok(counts)
}

#[derive(Debug)]
struct Config {
    debug: bool,
    timeout: Duration,
}"#;

let highlighted = highlighter.highlight(rust_code, "rust", Theme::Monokai)?;
println!("Highlighted Rust code:\n{:?}", highlighted);
```

### Python Code
```rust
use oak_highlight::{OakHighlighter, Theme, ExportFormat, Highlighter};

let highlighter = OakHighlighter::new();
let python_code = r#"import asyncio
import aiohttp
from typing import List, Optional

async def fetch_data(urls: List[str]) -> List[str]:
    """Fetch data from multiple URLs concurrently."""
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_single(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
        return results

async def fetch_single(session: aiohttp.ClientSession, url: str) -> Optional[str]:
    try:
        async with session.get(url) as response:
            return await response.text()
    except aiohttp.ClientError as e:
        print(f"Error fetching {url}: {e}")
        return None"#;

let highlighted = highlighter.highlight_format(
    python_code, 
    "python", 
    Theme::VSCode, 
    ExportFormat::Html
)?;
println!("HTML highlighted Python code:\n{}", highlighted);
```

### JavaScript Code
```rust
use oak_highlight::{OakHighlighter, Theme};

let highlighter = OakHighlighter::new();
let js_code = r#"class ApiClient {
    constructor(baseURL) {
        this.baseURL = baseURL;
        this.headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        };
    }
    
    async get(endpoint) {
        const response = await fetch(`${this.baseURL}${endpoint}`, {
            method: 'GET',
            headers: this.headers
        });
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        
        return await response.json();
    }
    
    async post(endpoint, data) {
        return fetch(`${this.baseURL}${endpoint}`, {
            method: 'POST',
            headers: this.headers,
            body: JSON.stringify(data)
        });
    }
}"#;

let highlighted = highlighter.highlight(js_code, "javascript", Theme::OneDarkPro)?;
println!("Highlighted JavaScript code:\n{}", highlighted);
```

## 🔧 Advanced Features

### Custom Themes
```rust
use oak_highlight::{Highlighter, Theme, TokenStyle, Color};

let mut highlighter = Highlighter::new();

// Create a custom theme
let custom_theme = Theme::Custom {
    name: "MyTheme".to_string(),
    background: Color::Rgb(40, 42, 54),
    foreground: Color::Rgb(248, 248, 242),
    styles: vec![
        (TokenStyle::Keyword, Color::Rgb(255, 121, 198)),
        (TokenStyle::String, Color::Rgb(241, 250, 140)),
        (TokenStyle::Comment, Color::Rgb(98, 114, 164)),
        (TokenStyle::Function, Color::Rgb(80, 250, 123)),
        (TokenStyle::Number, Color::Rgb(189, 147, 249)),
    ]
};

let code = "fn main() { println!(\"Hello\"); }";
let highlighted = highlighter.highlight(code, "rust", custom_theme)?;
```

### Language Detection
```rust
use oak_highlight::{Highlighter, LanguageDetector};

let highlighter = Highlighter::new();
let detector = LanguageDetector::new();

// Detect language from file extension
let language = detector.detect_from_extension(".py")?;
println!("Detected language: {}", language);

// Detect language from code content
let code = "def fibonacci(n):\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2)";
let language = detector.detect_from_content(code)?;
println!("Detected language from content: {}", language);
```

### Batch Processing
```rust
use oak_highlight::{Highlighter, BatchProcessor};
use std::collections::HashMap;

let highlighter = Highlighter::new();
let mut processor = BatchProcessor::new(highlighter);

let mut files = HashMap::new();
files.insert("main.rs", "fn main() { println!(\"Hello\"); }");
files.insert("script.py", "print('Hello from Python')");
files.insert("app.js", "console.log('Hello from JavaScript');");

let results = processor.highlight_batch(files, Theme::VSCode)?;
for (filename, highlighted) in results {
    println!("Highlighted {}:\n{}", filename, highlighted);
}
```

### Performance Monitoring
```rust
use oak_highlight::{Highlighter, PerformanceStats};

let mut highlighter = Highlighter::new();
highlighter.enable_performance_monitoring(true);

let code = r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("Number: {}", n);
    }
}"#;

let highlighted = highlighter.highlight(code, "rust", Theme::Monokai)?;

if let Some(stats) = highlighter.get_performance_stats() {
    println!("Tokenization time: {:?}", stats.tokenization_time);
    println!("Highlighting time: {:?}", stats.highlighting_time);
    println!("Total tokens: {}", stats.token_count);
}
```

## 🏗️ Supported Languages

Oak of highlight supports syntax highlighting for 100+ programming languages including:

- **Systems Languages**: Rust, C, C++, Go, Zig
- **Web Technologies**: JavaScript, TypeScript, HTML, CSS, JSON
- **Scripting Languages**: Python, Ruby, Perl, Bash, PowerShell
- **Functional Languages**: Haskell, OCaml, F#, Elixir
- **JVM Languages**: Java, Kotlin, Scala, Groovy
- **.NET Languages**: C#, F#, VB.NET
- **Mobile Development**: Swift, Kotlin, Dart
- **Data Formats**: JSON, YAML, TOML, XML, SQL
- **Configuration**: Dockerfile, Makefile, Git configs

## 📊 Performance

- **Fast Tokenization**: Optimized lexers for each language
- **Efficient Rendering**: Minimal allocations during highlighting
- **Caching**: Intelligent caching of tokenized results
- **Streaming**: Support for large files with streaming

## 🔗 Integration

Oak of highlight integrates seamlessly with:

- **Documentation Generators**: Highlight code examples in docs
- **Blog Platforms**: Syntax highlighting for code in blog posts
- **IDE Plugins**: Code highlighting for editors
- **Static Site Generators**: Highlight code in generated websites
- **Terminal Applications**: Colorful code display in terminals

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Basic syntax highlighting for different languages
- Custom theme creation and application
- HTML and terminal output generation
- Batch processing multiple files
- Performance optimization techniques

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Pex Syntax Highlighter** - Beautiful code highlighting for every language 🚀