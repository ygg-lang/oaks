#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// VOC 运行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VocMode {
    /// 编程模式 (类似 Valkyrie)
    Programming,
    /// 组件模式 (类似 HTML/Vue)
    Component,
}

impl Default for VocMode {
    fn default() -> Self {
        VocMode::Programming
    }
}

/// VOC 语言定义
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VocLanguage {
    /// 运行模式
    pub mode: VocMode,
}

impl VocLanguage {
    /// 创建新的语言配置
    pub fn new() -> Self {
        Self { mode: VocMode::default() }
    }

    /// 创建组件模式的语言配置
    pub fn component() -> Self {
        Self { mode: VocMode::Component }
    }
}

impl Language for VocLanguage {
    const NAME: &'static str = "voc";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VocTokenType;
    type ElementType = crate::parser::element_type::VocElementType;
    type TypedRoot = ();
}
