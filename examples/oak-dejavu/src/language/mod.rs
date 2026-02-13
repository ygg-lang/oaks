#![doc = include_str!("readme.md")]
use crate::{ast::DejavuRoot, lexer::token_type::DejavuSyntaxKind};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Syntax modes for Dejavu parser.
pub enum SyntaxMode {
    /// Programming mode: Standard .vk file
    Programming,
    /// Template mode: The entire file is a template environment
    Template,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Configuration for template delimiters.
pub struct TemplateConfig {
    /// Start of a control block (default: "<%")
    pub control_start: String,
    /// End of a control block (default: "%>")
    pub control_end: String,
    /// Start of an interpolation block (default: "{")
    pub interpolation_start: String,
    /// End of an interpolation block (default: "}")
    pub interpolation_end: String,
    /// Start of a template comment (default: "<#")
    pub comment_start: String,
    /// End of a template comment (default: "#>")
    pub comment_end: String,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            control_start: "<%".to_string(),
            control_end: "%>".to_string(),
            interpolation_start: "{".to_string(),
            interpolation_end: "}".to_string(),
            comment_start: "<#".to_string(),
            comment_end: "#>".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Dejavu programming language definition.
pub struct DejavuLanguage {
    /// Current syntax mode
    pub syntax_mode: SyntaxMode,
    /// Template configuration
    pub template: TemplateConfig,
}

impl Default for DejavuLanguage {
    fn default() -> Self {
        Self { 
            syntax_mode: SyntaxMode::Template,
            template: TemplateConfig::default(),
        }
    }
}

impl Language for DejavuLanguage {
    const NAME: &'static str = "dejavu";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = DejavuSyntaxKind;
    type ElementType = DejavuSyntaxKind;
    type TypedRoot = DejavuRoot;
}
