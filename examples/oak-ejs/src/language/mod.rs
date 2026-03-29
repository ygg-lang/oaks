/// EJS Language module
///
/// This module defines the language characteristics and configuration for EJS templates.
/// EJS (Embedded JavaScript) is a simple templating language that lets you generate HTML markup
/// with plain JavaScript.
use oak_core::language::{Language, LanguageCategory};

use crate::{lexer::token_type::EjsTokenType, parser::element_type::EjsElementType};

pub use crate::ast::EjsRoot;

/// Language definition for EJS templates
///
/// EJS is a templating language that embeds JavaScript code within HTML using special delimiters.
/// The default delimiters are `<%` and `%>`, with various modifiers for different output modes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EjsLanguage {
    /// The opening delimiter for EJS tags
    ///
    /// Default: `"<%"`
    ///
    /// This delimiter marks the beginning of an EJS code block.
    /// For example, with the default `<%`, you would write:
    /// - `<% code %>` for control flow
    /// - `<%= value %>` for escaped output
    /// - `<%- value %>` for raw output
    /// - `<%# comment %>` for comments
    pub open_delimiter: String,

    /// The closing delimiter for EJS tags
    ///
    /// Default: `"%>"`
    ///
    /// This delimiter marks the end of an EJS code block.
    pub close_delimiter: String,

    /// The marker for escaped output
    ///
    /// Default: `"="`
    ///
    /// When this marker appears after the opening delimiter, the expression
    /// is evaluated and the result is HTML-escaped before being output.
    /// Example: `<%= user.name %>` outputs the escaped value of `user.name`.
    pub output_escape: String,

    /// The marker for raw output
    ///
    /// Default: `"-"`
    ///
    /// When this marker appears after the opening delimiter, the expression
    /// is evaluated and the result is output without any escaping.
    /// Example: `<%- rawHtml %>` outputs the raw HTML content.
    /// Use with caution as this can expose XSS vulnerabilities.
    pub output_raw: String,

    /// The marker for comments
    ///
    /// Default: `"#"`
    ///
    /// When this marker appears after the opening delimiter, the content
    /// is treated as a comment and not rendered in the output.
    /// Example: `<%# This is a comment %>` produces no output.
    pub comment_marker: String,
}

impl Default for EjsLanguage {
    fn default() -> Self {
        Self { open_delimiter: "<%".to_string(), close_delimiter: "%>".to_string(), output_escape: "=".to_string(), output_raw: "-".to_string(), comment_marker: "#".to_string() }
    }
}

impl Language for EjsLanguage {
    const NAME: &'static str = "ejs";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = EjsTokenType;
    type ElementType = EjsElementType;
    type TypedRoot = EjsRoot;
}

impl EjsLanguage {
    /// Creates a new EJS language instance with default configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an EJS language instance with custom delimiters
    ///
    /// # Arguments
    ///
    /// * `open` - The opening delimiter (e.g., `"<%"`)
    /// * `close` - The closing delimiter (e.g., `"%>"`)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ejs = EjsLanguage::with_delimiters("<?", "?>");
    /// ```
    pub fn with_delimiters(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self { open_delimiter: open.into(), close_delimiter: close.into(), ..Self::default() }
    }
}
