#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Fortran language configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FortranLanguage {
    /// Whether to enable Fortran 2008 support.
    pub fortran_2008: bool,
    /// Whether to enable Fortran 2018 support.
    pub fortran_2018: bool,
    /// Whether to use fixed format (Fortran 77 style).
    pub fixed_format: bool,
    /// Whether to enable OpenMP support.
    pub openmp: bool,
    /// Whether to enable Coarray support.
    pub coarray: bool,
}

impl FortranLanguage {
    /// Creates a new Fortran language configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FortranLanguage {
    fn default() -> Self {
        Self { fortran_2008: true, fortran_2018: false, fixed_format: false, openmp: false, coarray: false }
    }
}

impl Language for FortranLanguage {
    const NAME: &'static str = "fortran";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::FortranTokenType;
    type ElementType = crate::parser::element_type::FortranElementType;
    type TypedRoot = crate::ast::FortranRoot;
}
