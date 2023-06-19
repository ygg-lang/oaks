use crate::{ast::TypstRoot, kind::TypstSyntaxKind};
use oak_core::Language;

/// Typst 语言配置
pub struct TypstLanguage {
    /// 是否启用数学模式
    pub math_mode: bool,
    /// 是否支持脚本
    pub scripting: bool,
    /// 是否启用严格模式
    pub strict: bool,
    /// 目标 Typst 版本
    pub target: TypstVersion,
    /// 是否允许实验性语法
    pub experimental: bool,
}

/// Typst 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypstVersion {
    V0_1,
    V0_2,
    V0_3,
    V0_4,
    V0_5,
    V0_6,
    V0_7,
    V0_8,
    V0_9,
    V0_10,
    V0_11,
    Latest,
}

impl TypstLanguage {
    /// 创建标准 Typst 配置
    pub fn standard() -> Self {
        Self { math_mode: false, scripting: false, strict: false, target: TypstVersion::V0_11, experimental: false }
    }

    /// 创建支持数学模式的 Typst 配置
    pub fn with_math() -> Self {
        Self { math_mode: true, scripting: false, strict: false, target: TypstVersion::V0_11, experimental: false }
    }

    /// 创建支持脚本的 Typst 配置
    pub fn with_scripting() -> Self {
        Self { math_mode: false, scripting: true, strict: false, target: TypstVersion::V0_11, experimental: false }
    }

    /// 创建严格模式 Typst 配置
    pub fn strict() -> Self {
        Self { math_mode: false, scripting: false, strict: true, target: TypstVersion::V0_11, experimental: false }
    }

    /// 创建实验性语法的 Typst 配置
    pub fn experimental() -> Self {
        Self { math_mode: true, scripting: true, strict: true, target: TypstVersion::Latest, experimental: true }
    }
}

impl Language for TypstLanguage {
    type SyntaxKind = TypstSyntaxKind;
    type TypedRoot = TypstRoot;
}
