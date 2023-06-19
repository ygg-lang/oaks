# Oak Nginx Parser

[![Crates.io](https://img.shields.io/crates/v/oak-nginx.svg)](https://crates.io/crates/oak-nginx)
[![Documentation](https://docs.rs/oak-nginx/badge.svg)](https://docs.rs/oak-nginx)
[![License](https://img.shields.io/crates/l/oak-nginx.svg)](https://github.com/yourusername/oak-nginx#license)

A comprehensive Nginx configuration parser for the Oak parsing framework, providing robust parsing capabilities for Nginx server blocks, directives, and configuration syntax.

## Features

- **Complete Nginx Syntax**: Parse server blocks, location blocks, upstream blocks, and all standard directives
- **Directive Support**: Handle all Nginx directives including core, HTTP, server, and location contexts
- **Nested Blocks**: Support for complex nested configuration structures
- **Variable Parsing**: Parse Nginx variables with proper interpolation support
- **Comments**: Preserve and parse inline and block comments
- **Include Files**: Support for include directives and file references
- **Error Handling**: Detailed error messages with line and column information
- **AST Generation**: Rich Abstract Syntax Tree for configuration analysis and manipulation
- **Zero Dependencies**: Pure Rust implementation with no external dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oak-nginx = "0.1.0"
```

## Quick Start

```rust
use oak::Parser;
use oak_nginx::NginxLanguage;

fn main() {
    let config = r#"
        server {
            listen 80;
            server_name example.com www.example.com;
            
            location / {
                proxy_pass http://localhost:8080;
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
            }
            
            location /static {
                root /var/www/html;
                expires 1d;
            }
            
            error_page 404 /404.html;
            error_page 500 502 503 504 /50x.html;
        }
    "#;
    
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    
    match parser.parse(&config, &language) {
        Ok(ast) => {
            println!("Successfully parsed Nginx configuration!");
            println!("AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Usage

### Parsing Complex Configurations

```rust
use oak::Parser;
use oak_nginx::NginxLanguage;

fn main() {
    let complex_config = r#"
        user nginx;
        worker_processes auto;
        error_log /var/log/nginx/error.log;
        pid /run/nginx.pid;
        
        events {
            worker_connections 1024;
            use epoll;
            multi_accept on;
        }
        
        http {
            include /etc/nginx/mime.types;
            default_type application/octet-stream;
            
            log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                           '$status $body_bytes_sent "$http_referer" '
                           '"$http_user_agent" "$http_x_forwarded_for"';
            
            access_log /var/log/nginx/access.log main;
            
            sendfile on;
            tcp_nopush on;
            tcp_nodelay on;
            keepalive_timeout 65;
            types_hash_max_size 2048;
            
            gzip on;
            gzip_vary on;
            gzip_min_length 1000;
            gzip_types text/plain text/css application/json application/javascript;
            
            upstream backend {
                server 127.0.0.1:8080 weight=3;
                server 127.0.0.1:8081 weight=2;
                server 127.0.0.1:8082 weight=1;
                keepalive 32;
            }
            
            server {
                listen 80;
                server_name api.example.com;
                
                location / {
                    proxy_pass http://backend;
                    proxy_http_version 1.1;
                    proxy_set_header Connection "";
                    proxy_set_header Host $host;
                    proxy_set_header X-Real-IP $remote_addr;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Forwarded-Proto $scheme;
                }
                
                location /health {
                    access_log off;
                    return 200 "healthy\n";
                    add_header Content-Type text/plain;
                }
            }
        }
    "#;
    
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    
    match parser.parse(&complex_config, &language) {
        Ok(ast) => {
            println!("Successfully parsed complex Nginx configuration!");
            // Process the AST for configuration validation or transformation
        }
        Err(error) => {
            eprintln!("Parse error at line {}: {}", error.line(), error.message());
        }
    }
}
```

### Error Handling with Context

```rust
use oak::Parser;
use oak_nginx::NginxLanguage;

fn parse_with_diagnostics(source: &str) {
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    
    match parser.parse(source, &language) {
        Ok(ast) => {
            println!("Parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error at line {}, column {}", error.line(), error.column());
            eprintln!("Error: {}", error.message());
            
            // Show context around the error
            let lines: Vec<&str> = source.lines().collect();
            if error.line() > 0 && error.line() <= lines.len() {
                eprintln!("Context:");
                eprintln!("  {}", lines[error.line() - 1]);
                eprintln!("  {}^", " ".repeat(error.column()));
            }
        }
    }
}
```

### Configuration Validation

```rust
use oak::Parser;
use oak_nginx::NginxLanguage;

fn validate_nginx_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(config_path)?;
    
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    
    match parser.parse(&config, &language) {
        Ok(ast) => {
            println!("Configuration kind is valid!");
            
            // You can now analyze the AST for semantic issues
            check_semantic_errors(&ast)?;
            
            Ok(())
        }
        Err(error) => {
            Err(format!("Syntax error at line {}: {}", error.line(), error.message()).into())
        }
    }
}
```

## AST Structure

The parser generates a comprehensive AST with the following main node types:

- **Configuration**: Top-level container for all Nginx configuration
- **Directive**: Individual configuration directives with parameters
- **Block**: Nested configuration blocks (server, location, upstream, etc.)
- **Context**: Configuration contexts (events, http, server, location)
- **Parameter**: Directive parameters including variables and literals
- **Variable**: Nginx variables with `$` prefix
- **Comment**: Inline and block comments
- **Include**: Include file references

### Block Types

- **Main Context**: Global directives
- **Events Block**: Connection handling configuration
- **HTTP Block**: HTTP server configuration
- **Server Block**: Virtual server configuration
- **Location Block**: URL location configuration
- **Upstream Block**: Load balancing configuration
- **Mail Block**: Mail proxy configuration

## Performance

- **Zero-copy parsing**: Minimal string allocations during parsing
- **Streaming support**: Parse large configuration files incrementally
- **Error recovery**: Continue parsing after encountering errors
- **Memory efficient**: Compact AST representation
- **Fast validation**: Quick syntax validation for large configs

## Integration

The parser integrates seamlessly with the broader Oak ecosystem:

```rust
use oak::Parser;
use oak_nginx::NginxLanguage;

// Use with other Oak tools for analysis and transformation
fn analyze_nginx_config(source: &str) -> Result<Analysis, ParseError> {
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    let ast = parser.parse(source, &language)?;
    
    // Perform semantic analysis, validation, or transformations
    perform_analysis(ast)
}

// Convert to other formats
fn convert_to_json(source: &str) -> Result<String, ParseError> {
    let mut parser = Parser::new();
    let language = NginxLanguage::new();
    let ast = parser.parse(source, &language)?;
    
    serde_json::to_string_pretty(&ast).map_err(|e| e.into())
}
```

## Use Cases

- **Configuration Validation**: Validate Nginx configuration syntax before deployment
- **Configuration Analysis**: Analyze server configurations for security and performance issues
- **Migration Tools**: Convert configurations between different formats or versions
- **IDE Support**: Provide syntax highlighting and error checking for Nginx configs
- **Automation**: Generate or modify configurations programmatically
- **Monitoring**: Parse and analyze configuration changes

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.