#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Objective-C language definition.
pub struct ObjectiveCLanguage {
    /// Whether ARC (Automatic Reference Counting) is enabled.
    pub arc_enabled: bool,
    /// Whether strict mode is enabled.
    pub strict_mode: bool,
}

impl ObjectiveCLanguage {
    /// Creates a new Objective-C language definition.
    pub fn new() -> Self {
        Self { arc_enabled: true, strict_mode: false }
    }

    /// Sets whether ARC is enabled.
    pub fn with_arc(mut self, enabled: bool) -> Self {
        self.arc_enabled = enabled;
        self
    }

    /// Sets whether strict mode is enabled.
    pub fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }
}

impl Default for ObjectiveCLanguage {
    fn default() -> Self {
        Self { arc_enabled: true, strict_mode: false }
    }
}

impl Language for ObjectiveCLanguage {
    const NAME: &'static str = "objective-c";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::ObjectiveCTokenType;
    type ElementType = crate::parser::element_type::ObjectiveCElementType;
    type TypedRoot = crate::ast::ObjectiveCRoot;
}
