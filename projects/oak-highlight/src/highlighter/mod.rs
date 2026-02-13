use crate::exporters::Exporter;
use core::range::Range;
use oak_core::{
    TokenType,
    language::{ElementRole, Language, TokenRole, UniversalElementRole, UniversalTokenRole},
    tree::{RedNode, RedTree},
    visitor::Visitor,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    string::{String, ToString},
    vec::Vec,
};

/// Highlight style configuration for visual text formatting.
///
/// This struct defines the visual appearance of highlighted text segments,
/// including colors, font weight, and text decorations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightStyle {
    /// Foreground text color in hex format (e.g., "#FF0000" for red)
    pub color: Option<String>,
    /// Background color in hex format (e.g., "#FFFF00" for yellow)
    pub background_color: Option<String>,
    /// Whether text should be displayed in bold
    pub bold: bool,
    /// Whether text should be displayed in italic
    pub italic: bool,
    /// Whether text should be underlined
    pub underline: bool,
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self { color: None, background_color: None, bold: false, italic: false, underline: false }
    }
}

/// Highlight theme configuration containing style definitions for different roles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightTheme {
    /// Theme name identifier
    pub name: String,
    /// Style mapping for various scopes.
    /// Scopes are dot-separated strings (e.g., "keyword.control.rust").
    pub styles: HashMap<String, HighlightStyle>,
}

impl Default for HighlightTheme {
    fn default() -> Self {
        let mut styles = HashMap::new();

        // Token Styles (Standard TextMate-like Scopes)
        styles.insert("keyword".to_string(), HighlightStyle { color: Some("#0000FF".to_string()), bold: true, ..Default::default() });
        styles.insert("keyword.operator".to_string(), HighlightStyle { color: Some("#800080".to_string()), ..Default::default() });
        styles.insert("variable.other".to_string(), HighlightStyle { color: Some("#001080".to_string()), ..Default::default() });
        styles.insert("constant".to_string(), HighlightStyle { color: Some("#098658".to_string()), ..Default::default() });
        styles.insert("constant.character.escape".to_string(), HighlightStyle { color: Some("#FF6600".to_string()), ..Default::default() });
        styles.insert("punctuation".to_string(), HighlightStyle { color: Some("#000080".to_string()), ..Default::default() });
        styles.insert("comment".to_string(), HighlightStyle { color: Some("#808080".to_string()), italic: true, ..Default::default() });
        styles.insert("punctuation.whitespace".to_string(), HighlightStyle::default());

        // Element Styles
        styles.insert("entity.name.function".to_string(), HighlightStyle { color: Some("#795E26".to_string()), bold: true, ..Default::default() });
        styles.insert("entity.name.type".to_string(), HighlightStyle { color: Some("#267F99".to_string()), ..Default::default() });
        styles.insert("variable.other.declaration".to_string(), HighlightStyle { color: Some("#795E26".to_string()), ..Default::default() });
        styles.insert("comment.block.documentation".to_string(), HighlightStyle { color: Some("#008000".to_string()), italic: true, ..Default::default() });
        styles.insert("meta.preprocessor".to_string(), HighlightStyle { color: Some("#AF00DB".to_string()), ..Default::default() });
        styles.insert("entity.other.attribute-name".to_string(), HighlightStyle { color: Some("#AF00DB".to_string()), ..Default::default() });
        styles.insert("entity.other.attribute-name.key".to_string(), HighlightStyle { color: Some("#001080".to_string()), ..Default::default() });

        // Common
        styles.insert("invalid".to_string(), HighlightStyle { color: Some("#FF0000".to_string()), background_color: Some("#FFCCCC".to_string()), ..Default::default() });
        styles.insert("none".to_string(), HighlightStyle::default());

        Self { name: "default".to_string(), styles }
    }
}

impl HighlightTheme {
    /// Resolves the style for a given scope, with fallback to parent scopes.
    /// Example: "keyword.control.rust" -> "keyword.control" -> "keyword" -> None
    pub fn resolve_style(&self, scope: &str) -> HighlightStyle {
        let mut current_scope = scope;
        while !current_scope.is_empty() {
            if let Some(style) = self.styles.get(current_scope) {
                return style.clone();
            }
            if let Some(pos) = current_scope.rfind('.') { current_scope = &current_scope[..pos] } else { break }
        }
        self.styles.get("none").cloned().unwrap_or_default()
    }

    /// Resolves the style for multiple scopes, returning the best (most specific) match.
    /// This follows TextMate's specificity rules where the deepest match across all scopes wins.
    pub fn resolve_styles(&self, scopes: &[String]) -> HighlightStyle {
        let mut best_style = None;
        let mut best_depth = -1;

        for scope in scopes {
            let mut current_scope = scope.as_str();
            // Count segments for depth
            let mut depth = (current_scope.split('.').count()) as i32;

            while !current_scope.is_empty() {
                if let Some(style) = self.styles.get(current_scope) {
                    if depth > best_depth {
                        best_depth = depth;
                        best_style = Some(style.clone())
                    }
                    break; // Found the most specific match for this scope string
                }
                if let Some(pos) = current_scope.rfind('.') {
                    current_scope = &current_scope[..pos];
                    depth -= 1
                }
                else {
                    break;
                }
            }
        }

        best_style.unwrap_or_else(|| self.styles.get("none").cloned().unwrap_or_default())
    }

    pub fn get_token_style(&self, role: oak_core::UniversalTokenRole) -> HighlightStyle {
        use oak_core::TokenRole;
        self.resolve_style(role.name())
    }

    pub fn get_element_style(&self, role: oak_core::UniversalElementRole) -> HighlightStyle {
        use oak_core::ElementRole;
        self.resolve_style(role.name())
    }
}

/// Helper to get scopes for a token role.
fn get_token_scopes<R: TokenRole>(role: R, language: &str, category: oak_core::language::LanguageCategory) -> Vec<String> {
    let specific_name = role.name();
    let universal_role = role.universal();
    let universal_name = universal_role.name();
    let category_prefix = match category {
        oak_core::language::LanguageCategory::Markup => "markup",
        oak_core::language::LanguageCategory::Config => "config",
        oak_core::language::LanguageCategory::Programming => "source",
        oak_core::language::LanguageCategory::Dsl => "dsl",
        _ => "source",
    };

    let mut scopes = Vec::with_capacity(5);

    // 1. Language-specific scope (e.g., "keyword.control.rust")
    scopes.push(format!("{}.{}", specific_name, language));

    // 2. Base name scope (e.g., "keyword.control")
    if specific_name != universal_name {
        scopes.push(specific_name.to_string());
    }

    // 3. Category + Universal name (e.g., "source.keyword")
    scopes.push(format!("{}.{}", category_prefix, universal_name));

    // 4. Pure Universal name (e.g., "keyword")
    scopes.push(universal_name.to_string());

    // 5. Category + Language (e.g., "source.rust")
    scopes.push(format!("{}.{}", category_prefix, language));

    scopes
}

/// Helper to get scopes for an element role.
fn get_element_scopes<R: ElementRole>(role: R, language: &str, category: oak_core::language::LanguageCategory) -> Vec<String> {
    let specific_name = role.name();
    let universal_role = role.universal();
    let universal_name = universal_role.name();
    let category_prefix = match category {
        oak_core::language::LanguageCategory::Markup => "markup",
        oak_core::language::LanguageCategory::Config => "config",
        oak_core::language::LanguageCategory::Programming => "source",
        oak_core::language::LanguageCategory::Dsl => "dsl",
        _ => "source",
    };

    let mut scopes = Vec::with_capacity(5);

    // 1. Language-specific scope
    scopes.push(format!("{}.{}", specific_name, language));

    // 2. Base name scope
    if specific_name != universal_name {
        scopes.push(specific_name.to_string());
    }

    // 3. Category + Universal name
    scopes.push(format!("{}.{}", category_prefix, universal_name));

    // 4. Pure Universal name
    scopes.push(universal_name.to_string());

    // 5. Category + Language
    scopes.push(format!("{}.{}", category_prefix, language));

    scopes
}

/// Trait for providing scopes for highlighting.
pub trait ScopeProvider {
    fn scopes(&self, language: &str, category: oak_core::language::LanguageCategory) -> Vec<String>;
}

impl ScopeProvider for UniversalTokenRole {
    fn scopes(&self, language: &str, category: oak_core::language::LanguageCategory) -> Vec<String> {
        get_token_scopes(*self, language, category)
    }
}

impl ScopeProvider for UniversalElementRole {
    fn scopes(&self, language: &str, category: oak_core::language::LanguageCategory) -> Vec<String> {
        get_element_scopes(*self, language, category)
    }
}

/// A serializable span representing a range in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for HighlightSpan {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

/// A segment of highlighted text with associated style and content.
///
/// Represents a contiguous range of text that shares the same highlighting style.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightSegment<'a> {
    /// Byte range in the source text that this segment covers
    pub span: HighlightSpan,
    /// Visual style to apply to this text segment
    pub style: HighlightStyle,
    /// The actual text content of this segment
    pub text: Cow<'a, str>,
}

/// Result of token highlighting containing styled text segments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightResult<'a> {
    /// The collection of styled text segments.
    pub segments: Vec<HighlightSegment<'a>>,
    /// The original source text that was highlighted.
    pub source: Cow<'a, str>,
}

/// A visitor that traverses the syntax tree to generate highlighting segments.
pub struct HighlightVisitor<'a, 't> {
    /// The theme used for style resolution.
    pub theme: &'t HighlightTheme,
    /// The segments collected during traversal.
    pub segments: Vec<HighlightSegment<'a>>,
    /// The source text.
    pub source: &'a str,
}

impl<'a, 't, 'tree, L: Language> Visitor<'tree, L> for HighlightVisitor<'a, 't> {
    fn visit_node(&mut self, node: RedNode<'tree, L>) {
        // Elements usually don't have direct colors unless they override token styles.
        // For now, we follow the visitor pattern to traverse children.
        for child in node.children() {
            match child {
                RedTree::Node(n) => <Self as Visitor<L>>::visit_node(self, n),
                RedTree::Leaf(t) => <Self as Visitor<L>>::visit_token(self, t),
            }
        }
    }

    fn visit_token(&mut self, token: oak_core::tree::RedLeaf<L>) {
        // Use scopes for highlighting
        let scopes = get_token_scopes(token.kind.role(), L::NAME, L::CATEGORY);
        let style = self.theme.resolve_styles(&scopes);

        let text = &self.source[token.span.start..token.span.end];

        self.segments.push(HighlightSegment { span: HighlightSpan { start: token.span.start, end: token.span.end }, style, text: Cow::Borrowed(text) });
    }
}

/// Base trait for kind highlighters.
///
/// This trait defines the interface for kind highlighting implementations
/// that can analyze source code and produce styled text segments.
pub trait Highlighter {
    /// Highlight the given source code for a specific language and theme.
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: crate::themes::Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>>;
}

impl Highlighter for OakHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: crate::themes::Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        self.highlight(source, language, theme)
    }
}

/// The main highlighter implementation that coordinates the highlighting process.
///
/// # Example
///
/// ```rust
/// use oak_highlight::{OakHighlighter, Theme};
///
/// let highlighter = OakHighlighter::new();
/// let result = highlighter.highlight("fn main() {}", "rust", Theme::OneDarkPro).unwrap();
/// assert!(!result.segments.is_empty());
/// ```
pub struct OakHighlighter {
    pub theme: HighlightTheme,
}

impl Default for OakHighlighter {
    fn default() -> Self {
        Self { theme: HighlightTheme::default() }
    }
}

impl OakHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_theme(mut self, theme: HighlightTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Set theme by name using the predefined themes.
    pub fn theme(mut self, theme: crate::themes::Theme) -> Self {
        self.theme = theme.get_theme();
        self
    }

    /// Main highlight method matching README API.
    pub fn highlight<'a>(&self, source: &'a str, _language: &str, theme: crate::themes::Theme) -> oak_core::errors::ParseResult<HighlightResult<'a>> {
        let theme_config = theme.get_theme();

        // Default implementation just treats everything as a single segment for now
        // if no specific language parser is used.
        // In a real scenario, we'd look up a parser from a registry.
        let segments = vec![HighlightSegment { span: Range { start: 0, end: source.len() }.into(), style: theme_config.resolve_style("none"), text: Cow::Borrowed(source) }];

        Ok(HighlightResult { segments, source: Cow::Borrowed(source) })
    }

    pub fn highlight_with_language<'a, L, P, LX>(&self, source: &'a str, theme: crate::themes::Theme, parser: &P, _lexer: &LX) -> oak_core::errors::ParseResult<HighlightResult<'a>>
    where
        L: Language + Send + Sync + 'static,
        P: oak_core::parser::Parser<L>,
        LX: oak_core::Lexer<L>,
        L::ElementType: From<L::TokenType>,
    {
        let theme_config = theme.get_theme();
        let source_text = oak_core::source::SourceText::new(source.to_string());
        let mut cache = oak_core::parser::session::ParseSession::<L>::new(1024);
        let parse_result = parser.parse(&source_text, &[], &mut cache);

        let mut visitor = HighlightVisitor { theme: &theme_config, segments: Vec::new(), source };

        let root_node = parse_result.result.map_err(|e| e)?;
        let red_root = RedNode::new(root_node, 0);

        <HighlightVisitor<'a, '_> as Visitor<L>>::visit_node(&mut visitor, red_root);

        Ok(HighlightResult { segments: visitor.segments, source: Cow::Borrowed(source) })
    }

    /// Highlight and format to a string directly.
    pub fn highlight_format(&self, source: &str, language: &str, theme: crate::themes::Theme, format: crate::exporters::ExportFormat) -> oak_core::errors::ParseResult<String> {
        let result = self.highlight(source, language, theme)?;

        let content = match format {
            crate::exporters::ExportFormat::Html => crate::exporters::HtmlExporter::new(true, true).export(&result),
            crate::exporters::ExportFormat::Json => crate::exporters::JsonExporter { pretty: true }.export(&result),
            crate::exporters::ExportFormat::Ansi => crate::exporters::AnsiExporter.export(&result),
            crate::exporters::ExportFormat::Css => crate::exporters::CssExporter.export(&result),
            _ => {
                return Err(oak_core::errors::OakError::unsupported_format(format!("{format:?}")));
            }
        };

        Ok(content)
    }
}
