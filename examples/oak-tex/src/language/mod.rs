use crate::kind::TexSyntaxKind;
use oak_core::Language;

/// TeX 语言配置
pub struct TexLanguage {
    /// TeX 版本
    pub version: TexVersion,
    /// 是否支持数学模式
    pub math_mode: bool,
    /// 是否支持扩展包
    pub packages: bool,
    /// 是否支持自定义命令
    pub custom_commands: bool,
    /// 是否启用严格模式
    pub strict: bool,
}

/// TeX 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexVersion {
    /// 原始 TeX
    Tex,
    /// LaTeX
    LaTeX,
    /// LaTeX2e
    LaTeX2e,
    /// XeTeX
    XeTeX,
    /// LuaTeX
    LuaTeX,
    /// pdfTeX
    PdfTeX,
}

impl TexLanguage {
    /// 创建标准 LaTeX 配置
    pub fn standard() -> Self {
        Self { version: TexVersion::LaTeX2e, math_mode: true, packages: true, custom_commands: true, strict: false }
    }

    /// 创建原始 TeX 配置
    pub fn tex() -> Self {
        Self { version: TexVersion::Tex, math_mode: true, packages: false, custom_commands: false, strict: true }
    }

    /// 创建 XeTeX 配置
    pub fn xetex() -> Self {
        Self { version: TexVersion::XeTeX, math_mode: true, packages: true, custom_commands: true, strict: false }
    }

    /// 创建 LuaTeX 配置
    pub fn luatex() -> Self {
        Self { version: TexVersion::LuaTeX, math_mode: true, packages: true, custom_commands: true, strict: false }
    }

    /// 创建 pdfTeX 配置
    pub fn pdftex() -> Self {
        Self { version: TexVersion::PdfTeX, math_mode: true, packages: true, custom_commands: true, strict: false }
    }

    /// 创建严格模式配置
    pub fn strict() -> Self {
        Self { version: TexVersion::LaTeX2e, math_mode: true, packages: true, custom_commands: false, strict: true }
    }

    /// 创建支持扩展的配置
    pub fn with_extensions() -> Self {
        Self { version: TexVersion::LaTeX2e, math_mode: true, packages: true, custom_commands: true, strict: false }
    }
}

impl Language for TexLanguage {
    type SyntaxKind = TexSyntaxKind;
}
