#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// F# language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FSharpLanguage {
    /// Whether to enable F# 4.0 features.
    pub fsharp_4_0: bool,
    /// Whether to enable F# 4.1 features.
    pub fsharp_4_1: bool,
    /// Whether to enable F# 4.5 features.
    pub fsharp_4_5: bool,
    /// Whether to enable F# 5.0 features.
    pub fsharp_5_0: bool,
    /// Whether to enable F# 6.0 features.
    pub fsharp_6_0: bool,
    /// Whether to enable F# 7.0 features.
    pub fsharp_7_0: bool,
    /// Whether to enable computation expressions.
    pub computation_expressions: bool,
    /// Whether to enable type providers.
    pub type_providers: bool,
    /// Whether to enable async workflows.
    pub async_workflows: bool,
    /// Whether to enable query expressions.
    pub query_expressions: bool,
}

impl Default for FSharpLanguage {
    fn default() -> Self {
        Self { fsharp_4_0: true, fsharp_4_1: true, fsharp_4_5: true, fsharp_5_0: true, fsharp_6_0: true, fsharp_7_0: true, computation_expressions: true, type_providers: true, async_workflows: true, query_expressions: true }
    }
}

impl FSharpLanguage {
    /// Creates a new F# language configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables all F# features.
    pub fn with_all_features(mut self) -> Self {
        self.fsharp_4_0 = true;
        self.fsharp_4_1 = true;
        self.fsharp_4_5 = true;
        self.fsharp_5_0 = true;
        self.fsharp_6_0 = true;
        self.fsharp_7_0 = true;
        self.computation_expressions = true;
        self.type_providers = true;
        self.async_workflows = true;
        self.query_expressions = true;
        self
    }

    /// Sets the F# version.
    pub fn with_version(mut self, major: u8, minor: u8) -> Self {
        match (major, minor) {
            (4, 0) => self.fsharp_4_0 = true,
            (4, 1) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true
            }
            (4, 5) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true
            }
            (5, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true
            }
            (6, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true;
                self.fsharp_6_0 = true
            }
            (7, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true;
                self.fsharp_6_0 = true;
                self.fsharp_7_0 = true
            }
            _ => {}
        }
        self
    }

    /// Enables or disables computation expressions.
    pub fn with_computation_expressions(mut self, enabled: bool) -> Self {
        self.computation_expressions = enabled;
        self
    }

    /// Enables or disables type providers.
    pub fn with_type_providers(mut self, enabled: bool) -> Self {
        self.type_providers = enabled;
        self
    }

    /// Enables or disables async workflows.
    pub fn with_async_workflows(mut self, enabled: bool) -> Self {
        self.async_workflows = enabled;
        self
    }

    /// Enables or disables query expressions.
    pub fn with_query_expressions(mut self, enabled: bool) -> Self {
        self.query_expressions = enabled;
        self
    }
}

impl Language for FSharpLanguage {
    const NAME: &'static str = "fsharp";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::FSharpTokenType;
    type ElementType = crate::parser::element_type::FSharpElementType;
    type TypedRoot = crate::ast::FSharpRoot;
}
