use crate::kind::FortranSyntaxKind;
use oak_core::Language;

/// Fortran 语言配置
#[derive(Debug, Clone)]
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

impl Default for FortranLanguage {
    fn default() -> Self {
        Self { fortran_2008: true, fortran_2018: false, fixed_format: false, openmp: false, coarray: false }
    }
}

pub struct FortranRoot;

impl Language for FortranLanguage {
    type SyntaxKind = FortranSyntaxKind;
    type TypedRoot = FortranRoot;
}
