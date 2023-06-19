use crate::{ast::TypeScriptRoot, kind::TypeScriptSyntaxKind};
use oak_core::Language;

/// TypeScript 语言配置
pub struct TypeScriptLanguage {
    /// 是否支持 JSX 语法
    pub jsx: bool,
    /// 是否支持装饰器
    pub decorators: bool,
    /// 是否启用严格模式
    pub strict: bool,
    /// 目标 ECMAScript 版本
    pub target: EcmaVersion,
    /// 是否允许实验性语法
    pub experimental: bool,
}

/// ECMAScript 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcmaVersion {
    ES3,
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
    ES2022,
    ESNext,
}

impl TypeScriptLanguage {
    /// 创建标准 TypeScript 配置
    pub fn standard() -> Self {
        Self { jsx: false, decorators: false, strict: false, target: EcmaVersion::ES2020, experimental: false }
    }

    /// 创建支持 JSX TypeScript 配置
    pub fn with_jsx() -> Self {
        Self { jsx: true, decorators: false, strict: false, target: EcmaVersion::ES2020, experimental: false }
    }

    /// 创建支持装饰器的 TypeScript 配置
    pub fn with_decorators() -> Self {
        Self { jsx: false, decorators: true, strict: false, target: EcmaVersion::ES2020, experimental: false }
    }

    /// 创建严格模式TypeScript 配置
    pub fn strict() -> Self {
        Self { jsx: false, decorators: false, strict: true, target: EcmaVersion::ES2020, experimental: false }
    }

    /// 创建实验性语法的 TypeScript 配置
    pub fn experimental() -> Self {
        Self { jsx: true, decorators: true, strict: true, target: EcmaVersion::ESNext, experimental: true }
    }
}

impl Language for TypeScriptLanguage {
    type SyntaxKind = TypeScriptSyntaxKind;
    type TypedRoot = TypeScriptRoot;
}
