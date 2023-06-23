//! JavaScript language implementation.

use oak_core::language::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// JavaScript language implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JavaScriptLanguage {
    /// Whether to allow JSX syntax.
    pub jsx: bool,
    /// Whether to allow TypeScript syntax.
    pub typescript: bool,
    /// Whether to allow experimental features.
    pub experimental: bool,
    /// Whether to enable strict mode.
    pub strict_mode: bool,
    /// ECMAScript version.
    pub ecma_version: EcmaVersion,
}

/// ECMAScript version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EcmaVersion {
    /// ES3
    ES3,
    /// ES5
    ES5,
    /// ES2015 (ES6)
    ES2015,
    /// ES2016
    ES2016,
    /// ES2017
    ES2017,
    /// ES2018
    ES2018,
    /// ES2019
    ES2019,
    /// ES2020
    ES2020,
    /// ES2021
    ES2021,
    /// ES2022
    ES2022,
    /// ES2023
    ES2023,
    /// Latest supported version
    Latest,
}

impl JavaScriptLanguage {
    /// Creates a new JavaScript language configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard JavaScript language instance.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Creates a modern (ES2015+) JavaScript language instance.
    pub fn modern() -> Self {
        Self { jsx: false, typescript: false, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }

    /// Creates a JavaScript language instance with JSX support.
    pub fn jsx() -> Self {
        Self { jsx: true, typescript: false, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }

    /// Creates a TypeScript language instance.
    pub fn typescript() -> Self {
        Self { jsx: false, typescript: true, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }
}

impl Default for JavaScriptLanguage {
    fn default() -> Self {
        Self { jsx: false, typescript: false, experimental: false, strict_mode: false, ecma_version: EcmaVersion::ES2015 }
    }
}

impl Language for JavaScriptLanguage {
    const NAME: &'static str = "javascript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::JavaScriptTokenType;
    type ElementType = crate::parser::element_type::JavaScriptElementType;
    type TypedRoot = crate::ast::JavaScriptRoot;
}
