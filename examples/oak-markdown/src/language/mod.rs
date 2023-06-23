#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for the Markdown language features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkdownLanguage {
    /// Enable math formulas.
    ///
    /// Example: `$a^2 + b^2 = c^2$` or `$$E = mc^2$$`
    pub allow_math: bool,
    /// Enable tables.
    ///
    /// Example:
    /// | Header |
    /// | ------ |
    /// | Cell   |
    pub allow_tables: bool,
    /// Enable task lists.
    ///
    /// Example: `- [ ] Task` or `- [x] Done`
    pub allow_task_lists: bool,
    /// Enable strikethrough.
    ///
    /// Example: `~~deleted~~`
    pub allow_strikethrough: bool,
    /// Enable footnotes.
    ///
    /// Example: `[^1]` and `[^1]: Note`
    pub allow_footnotes: bool,
    /// Enable front matter (YAML/TOML/JSON).
    ///
    /// Example:
    /// ---
    /// title: Hello
    /// ---
    pub allow_front_matter: bool,
    /// Enable definition lists.
    ///
    /// Example:
    /// Term
    /// : Definition
    pub allow_definition_lists: bool,
    /// Enable superscript and subscript.
    ///
    /// Example: `^sup^` or `~sub~`
    pub allow_sub_superscript: bool,
    /// Enable autolinks.
    ///
    /// Example: `<https://example.com>`
    pub allow_autolinks: bool,
    /// Enable abbreviations.
    ///
    /// Example: `*[HTML]: HyperText Markup Language`
    pub allow_abbreviations: bool,
    /// Enable indented code blocks.
    ///
    /// Example:
    ///     code block
    pub allow_indented_code_blocks: bool,
    /// Enable inline HTML tags.
    ///
    /// Example: `<div>` or `<!-- comment -->`
    pub allow_html: bool,
    /// Enable hard line breaks.
    ///
    /// Example: Two spaces at the end of a line or a backslash.
    pub allow_hard_line_breaks: bool,
    /// Enable GFM-style autolinks.
    ///
    /// Example: `https://example.com`
    pub allow_gfm_autolinks: bool,
    /// Enable ATX headings.
    ///
    /// Example: `# Heading`
    pub allow_headings: bool,
    /// Enable lists.
    ///
    /// Example: `- Item` or `1. Item`
    pub allow_lists: bool,
    /// Enable blockquotes.
    ///
    /// Example: `> Quote`
    pub allow_blockquotes: bool,
    /// Enable fenced code blocks.
    ///
    /// Example: ` ```rust `
    pub allow_fenced_code_blocks: bool,
    /// Enable horizontal rules.
    ///
    /// Example: `---` or `***`
    pub allow_horizontal_rules: bool,
    /// Enable Setext headings.
    ///
    /// Example:
    /// Heading
    /// =======
    pub allow_setext_headings: bool,
    /// Enable GFM Tagfilter.
    ///
    /// Filters certain HTML tags like `<script>`.
    pub allow_html_tagfilter: bool,
    /// Enable XML/TSX syntax.
    ///
    /// Example: `<Component />`
    pub allow_xml: bool,
}

impl Language for MarkdownLanguage {
    const NAME: &'static str = "markdown";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::MarkdownTokenType;
    type ElementType = crate::parser::element_type::MarkdownElementType;
    type TypedRoot = crate::ast::MarkdownRoot;
}

impl Default for MarkdownLanguage {
    fn default() -> Self {
        Self {
            allow_math: true,
            allow_tables: true,
            allow_task_lists: true,
            allow_strikethrough: true,
            allow_footnotes: true,
            allow_front_matter: true,
            allow_definition_lists: false,
            allow_sub_superscript: false,
            allow_autolinks: true,
            allow_abbreviations: false,
            allow_indented_code_blocks: true,
            allow_html: true,
            allow_hard_line_breaks: true,
            allow_gfm_autolinks: true,
            allow_headings: true,
            allow_lists: true,
            allow_blockquotes: true,
            allow_fenced_code_blocks: true,
            allow_horizontal_rules: true,
            allow_setext_headings: true,
            allow_html_tagfilter: false,
            allow_xml: false,
        }
    }
}
