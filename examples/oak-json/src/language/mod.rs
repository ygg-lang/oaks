use oak_core::language::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// JSON 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JsonLanguage {
    /// 是否允许尾随逗号
    pub trailing_comma: bool,
    /// 是否允许裸键（不带引号的键）
    pub bare_keys: bool,
    /// 是否允许单引号字符串
    pub single_quotes: bool,
    /// 是否允许注释
    pub comments: bool,
    /// 是否允许十六进制数字
    pub hex_numbers: bool,
    /// 是否允许无穷大和 NaN
    pub infinity_and_nan: bool,
}

impl JsonLanguage {
    /// 创建新的 JSON 语言实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建标准 JSON 语言实例
    pub fn standard() -> Self {
        Self::default()
    }

    /// 创建 JSON5 语言实例
    pub fn json5() -> Self {
        Self { trailing_comma: true, bare_keys: true, single_quotes: true, comments: true, hex_numbers: true, infinity_and_nan: true }
    }

    /// 创建宽松 JSON 语言实例
    pub fn relaxed() -> Self {
        Self { trailing_comma: true, bare_keys: true, single_quotes: true, comments: true, hex_numbers: true, infinity_and_nan: true }
    }
}

impl Default for JsonLanguage {
    fn default() -> Self {
        Self { trailing_comma: false, bare_keys: false, single_quotes: false, comments: false, hex_numbers: false, infinity_and_nan: false }
    }
}

impl Language for JsonLanguage {
    const NAME: &'static str = "json";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::JsonSyntaxKind;
    type ElementType = crate::kind::JsonSyntaxKind;
    type TypedRoot = crate::ast::JsonRoot;
}
