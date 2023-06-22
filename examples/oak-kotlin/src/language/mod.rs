use oak_core::language::{Language, LanguageCategory};

/// Kotlin 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KotlinLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许实验性特性
    pub experimental_features: bool,
}

impl KotlinLanguage {
    /// 创建标准 Kotlin 语言实例
    pub fn standard() -> Self {
        Self::default()
    }

    /// 创建启用实验性特性的 Kotlin 语言实例
    pub fn experimental() -> Self {
        Self { strict_mode: false, experimental_features: true }
    }

    /// 创建严格模式Kotlin 语言实例
    pub fn strict() -> Self {
        Self { strict_mode: true, experimental_features: false }
    }
}

impl Default for KotlinLanguage {
    fn default() -> Self {
        Self { strict_mode: false, experimental_features: false }
    }
}

impl Language for KotlinLanguage {
    const NAME: &'static str = "kotlin";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::KotlinSyntaxKind;
    type ElementType = crate::kind::KotlinSyntaxKind;
    type TypedRoot = crate::ast::KotlinRoot;
}
