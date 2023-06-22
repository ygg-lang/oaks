use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Fortran 语言配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FortranLanguage {
    /// 是否启用 Fortran 2008
    pub fortran_2008: bool,
    /// 是否启用 Fortran 2018
    pub fortran_2018: bool,
    /// 是否启用固定格式（Fortran 77 风格）
    pub fixed_format: bool,
    /// 是否启用 OpenMP 支持
    pub openmp: bool,
    /// 是否启用 Coarray 支持
    pub coarray: bool,
}

impl FortranLanguage {
    /// 创建新的 Fortran 语言实例
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

    type TokenType = crate::kind::FortranSyntaxKind;
    type ElementType = crate::kind::FortranSyntaxKind;
    type TypedRoot = ();
}
