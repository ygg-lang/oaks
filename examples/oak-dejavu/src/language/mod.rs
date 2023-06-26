use crate::{ast::DejavuRoot, lexer::DejavuTokenType, parser::DejavuElementType};
use oak_core::{Language, LanguageCategory};

/// Dejavu language configuration and metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DejavuLanguage {
    /// The syntax mode.
    pub syntax_mode: SyntaxMode,
    /// The template configuration.
    pub template: TemplateConfig,
}

/// Syntax mode for Dejavu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyntaxMode {
    /// Programming mode (pure code).
    Programming,
    /// Template mode (text with interpolations).
    Template,
}

/// Template configuration.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateConfig {
    /// Control flow start delimiter.
    pub control_start: String,
    /// Control flow end delimiter.
    pub control_end: String,
    /// Interpolation start delimiter.
    pub interpolation_start: String,
    /// Interpolation end delimiter.
    pub interpolation_end: String,
    /// Comment start delimiter.
    pub comment_start: String,
    /// Comment end delimiter.
    pub comment_end: String,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self { control_start: "<%".to_string(), control_end: "%>".to_string(), interpolation_start: "<%".to_string(), interpolation_end: "%>".to_string(), comment_start: "<*".to_string(), comment_end: "*>".to_string() }
    }
}

impl DejavuLanguage {
    /// Creates a new Dejavu language configuration.
    pub fn new() -> Self {
        Self { syntax_mode: SyntaxMode::Template, template: TemplateConfig::default() }
    }

    /// Sets the syntax mode.
    pub fn with_mode(mut self, mode: SyntaxMode) -> Self {
        self.syntax_mode = mode;
        self
    }

    /// Sets the template configuration.
    pub fn with_template(mut self, template: TemplateConfig) -> Self {
        self.template = template;
        self
    }
}

impl Default for DejavuLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl Language for DejavuLanguage {
    const NAME: &'static str = "dejavu";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = DejavuTokenType;
    type ElementType = DejavuElementType;
    type TypedRoot = DejavuRoot;
}
