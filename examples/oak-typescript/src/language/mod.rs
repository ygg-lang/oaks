#![doc = include_str!("readme.md")]
use crate::ast::TypeScriptRoot;
use oak_core::{Language, LanguageCategory};

/// TypeScript language configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeScriptLanguage {
    /// Whether to support JSX syntax.
    pub jsx: bool,
    /// Whether to support decorators.
    pub decorators: bool,
    /// Whether to enable strict mode.
    pub strict: bool,
    /// Target ECMAScript version.
    pub target: EcmaVersion,
    /// Whether to allow experimental syntax.
    pub experimental: bool,
}

impl Default for TypeScriptLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

/// ECMAScript version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EcmaVersion {
    /// ECMAScript 3 (1999).
    ES3,
    /// ECMAScript 5 (2009).
    ES5,
    /// ECMAScript 2015 (ES6).
    ES2015,
    /// ECMAScript 2016.
    ES2016,
    /// ECMAScript 2017.
    ES2017,
    /// ECMAScript 2018.
    ES2018,
    /// ECMAScript 2019.
    ES2019,
    /// ECMAScript 2020.
    ES2020,
    /// ECMAScript 2021.
    ES2021,
    /// ECMAScript 2022.
    ES2022,
    /// Latest ECMAScript version.
    ESNext,
}

impl TypeScriptLanguage {
    /// Creates a new TypeScript language configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a standard TypeScript configuration.
    pub fn standard() -> Self {
        Self { jsx: true, decorators: true, strict: false, target: EcmaVersion::ES2020, experimental: true }
    }

    /// Creates a TypeScript configuration with JSX support.
    pub fn with_jsx() -> Self {
        Self { jsx: true, decorators: false, strict: false, target: EcmaVersion::ES2020, experimental: false }
    }

    /// Creates a TypeScript configuration with decorator support.
    pub fn with_decorators() -> Self {
        Self { jsx: false, decorators: true, strict: false, target: EcmaVersion::ES2020, experimental: false }
    }

    /// Creates a strict mode TypeScript configuration.
    pub fn strict() -> Self {
        Self { jsx: false, decorators: false, strict: true, target: EcmaVersion::ES2020, experimental: false }
    }

    /// Creates a TypeScript configuration with experimental syntax.
    pub fn experimental() -> Self {
        Self { jsx: true, decorators: true, strict: true, target: EcmaVersion::ESNext, experimental: true }
    }
}

impl Language for TypeScriptLanguage {
    const NAME: &'static str = "typescript";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::TypeScriptTokenType;
    type ElementType = crate::parser::element_type::TypeScriptElementType;
    type TypedRoot = TypeScriptRoot;
}
