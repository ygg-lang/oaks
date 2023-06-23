#![doc = include_str!("readme.md")]
use crate::ast::TclRoot;
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tcl language configuration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TclLanguage {
    /// Tcl version.
    pub version: TclVersion,
    /// Whether to enable extended syntax.
    pub extensions: bool,
}

impl Default for TclLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl TclLanguage {
    /// Creates a new `TclLanguage` instance.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Tcl version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TclVersion {
    /// Tcl 8.0
    Tcl80,
    /// Tcl 8.1
    Tcl81,
    /// Tcl 8.2
    Tcl82,
    /// Tcl 8.3
    Tcl83,
    /// Tcl 8.4
    Tcl84,
    /// Tcl 8.5
    Tcl85,
    /// Tcl 8.6
    Tcl86,
    /// Tcl 8.7
    Tcl87,
    /// Tcl 9.0
    Tcl90,
}

impl TclLanguage {
    /// Creates a standard Tcl 8.6 configuration.
    pub fn standard() -> Self {
        Self { version: TclVersion::Tcl86, extensions: false }
    }

    /// Creates a Tcl 8.5 configuration.
    pub fn tcl85() -> Self {
        Self { version: TclVersion::Tcl85, extensions: false }
    }

    /// Creates a Tcl 8.6 configuration.
    pub fn tcl86() -> Self {
        Self { version: TclVersion::Tcl86, extensions: false }
    }

    /// Creates a Tcl 9.0 configuration.
    pub fn tcl90() -> Self {
        Self { version: TclVersion::Tcl90, extensions: true }
    }

    /// Creates a Tcl configuration with extended syntax.
    pub fn with_extensions() -> Self {
        Self { version: TclVersion::Tcl86, extensions: true }
    }
}

impl Language for TclLanguage {
    const NAME: &'static str = "tcl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::TclTokenType;
    type ElementType = crate::parser::element_type::TclElementType;
    type TypedRoot = TclRoot;
}
