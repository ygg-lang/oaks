use oak_core::language::Language;

/// JavaScript 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JavaScriptLanguage {
    /// 是否允许 JSX 语法
    pub jsx: bool,
    /// 是否允许 TypeScript 语法
    pub typescript: bool,
    /// 是否允许实验性语
    pub experimental: bool,
    /// 是否严格模式
    pub strict_mode: bool,
    /// ECMAScript 版本
    pub ecma_version: EcmaVersion,
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
    ES2023,
    Latest,
}

impl JavaScriptLanguage {
    /// 创建标准 JavaScript 语言实例
    pub fn standard() -> Self {
        Self::default()
    }

    /// 创建 ES2015+ JavaScript 语言实例
    pub fn modern() -> Self {
        Self { jsx: false, typescript: false, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }

    /// 创建支持 JSX JavaScript 语言实例
    pub fn jsx() -> Self {
        Self { jsx: true, typescript: false, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }

    /// 创建 TypeScript 语言实例
    pub fn typescript() -> Self {
        Self { jsx: false, typescript: true, experimental: false, strict_mode: true, ecma_version: EcmaVersion::Latest }
    }
}

impl Default for JavaScriptLanguage {
    fn default() -> Self {
        Self { jsx: false, typescript: false, experimental: false, strict_mode: false, ecma_version: EcmaVersion::ES2015 }
    }
}

impl Language for JavaScriptLanguage {
    type SyntaxKind = crate::kind::JavaScriptSyntaxKind;
    type TypedRoot = ();
}
