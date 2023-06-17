# Oak CSV Parser

[![Crates.io](https://img.shields.io/crates/v/oak-csv.svg)](https://crates.io/crates/oak-csv)
[![Documentation](https://docs.rs/oak-csv/badge.svg)](https://docs.rs/oak-csv)

High-performance incremental CSV parser for the oak ecosystem with flexible configuration, optimized for data processing and analysis.

## 🎯 Overview

Oak-csv is a robust parser for CSV, designed to handle complete CSV syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for data processing and analysis.

## ✨ Features

- **Complete CSV Syntax**: Supports all CSV features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_csv::CsvParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CsvParser::new();
    let csv_content = r#"
name,age,city,country
John Doe,25,New York,USA
Jane Smith,30,London,UK
Bob Johnson,35,Paris,France
    "#;
    
    let document = parser.parse_document(csv_content)?;
    println!("Parsed CSV document with {} records.", document.records.len());
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_csv::{CsvParser, ast::Document};

let parser = CsvParser::new();
let csv_content = r#"
product_id,product_name,price,stock
001,Smartphone,599.99,50
002,Laptop,1299.99,25
003,Headphones,199.99,100
"#;

let document = parser.parse_document(csv_content)?;
println!("Headers: {:?}", document.headers);
println!("Records: {}", document.records.len());
```

### Record Parsing
```rust
use oak_csv::{CsvParser, ast::Record};

let parser = CsvParser::new();
let csv_content = "Alice,28,Engineer,Seattle";

let record = parser.parse_record(csv_content)?;
println!("Fields: {:?}", record.fields);
println!("Field count: {}", record.fields.len());
```

### Field Parsing
```rust
use oak_csv::{CsvParser, ast::Field};

let parser = CsvParser::new();
let field_content = "\"John \"JD\" Doe\"";

let field = parser.parse_field(field_content)?;
println!("Field value: {}", field.value);
println!("Quoted: {}", field.is_quoted);
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_csv::{CsvParser, lexer::Token};

let parser = CsvParser::new();
let tokens = parser.tokenize("name,age,city\nJohn,25,NYC")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_csv::CsvParser;

let parser = CsvParser::new();
let invalid_csv = r#"
name,age,city
John,25,NYC
Jane,30  // Missing field
Bob,35,London,UK
"#;

match parser.parse_document(invalid_csv) {
    Ok(document) => println!("Parsed CSV document successfully."),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Document**: Root container for CSV documents
- **Record**: CSV records with field values
- **Field**: Individual CSV fields with optional quoting
- **Header**: Optional header row with column names
- **Delimiter**: Field delimiter (usually comma)
- **Quote**: Quote character for quoted fields

## 📊 Performance

- **Streaming**: Parse large CSV files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-csv integrates seamlessly with:

- **Data Processing**: Extract data from CSV files
- **Configuration Files**: Parse CSV configuration files
- **Data Analysis**: Process CSV data for analysis
- **IDE Support**: Language server protocol compatibility
- **ETL Pipelines**: CSV parsing for data transformation

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete CSV document parsing
- Record and field analysis
- Data extraction and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-csv) or open [issues](https://github.com/ygg-lang/oaks/issues).