#![doc = include_str!("readme.md")]
/// Highlighter module.
#[cfg(feature = "oak-highlight")]
pub mod highlighter;
/// MCP module.
/// LSP service implementation.
#[cfg(feature = "lsp")]
pub struct GsglLanguageService;
