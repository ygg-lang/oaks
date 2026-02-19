# 🚀 oak-mcp

[![Crates.io](https://img.shields.io/crates/v/oak-mcp.svg)](https://crates.io/crates/oak-mcp)
[![Documentation](https://docs.rs/oak-mcp/badge.svg)](https://docs.rs/oak-mcp)

**Model Context Protocol Integration for Oak** — Enable AI assistants to understand and analyze code through the MCP standard.

## 🎯 Why oak-mcp?

The Model Context Protocol (MCP) is an open standard that enables AI assistants to interact with development tools and codebases. `oak-mcp` bridges Oak's parsing capabilities with MCP, allowing AI agents to perform code analysis and understanding tasks.

## ✨ Key Features

- **🤖 MCP Server Implementation** — Standard MCP server exposing Oak's language analysis
- **📊 Code Analysis Tools** — Parsing, symbol extraction, and navigation as MCP tools
- **🔍 Semantic Understanding** — Query symbol definitions, references, and documentation
- **📁 Project-Wide Analysis** — Multi-file project understanding via `oak-vfs`
- **🌐 Language Agnostic** — Works with any Oak language parser

## 🏗️ Architecture

### MCP Tools Provided

| Tool | Description |
|------|-------------|
| `parse_file` | Parse source file and return AST structure |
| `find_symbols` | Search for symbols matching a query |
| `get_definition` | Get definition location of a symbol |
| `find_references` | Find all references to a symbol |
| `get_hover` | Get hover information for a position |

## 🔗 Ecosystem Integration

Integrates with `oak-core` for parsing, `oak-vfs` for file access, `oak-navigation` for code navigation, and any MCP-compatible AI assistant.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-mcp).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
