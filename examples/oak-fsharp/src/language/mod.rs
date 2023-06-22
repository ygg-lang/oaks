use oak_core::{Language, LanguageCategory};

/// F# 语言实现
#[derive(Debug, Clone)]
pub struct FSharpLanguage {
    /// 是否启用 F# 4.0
    pub fsharp_4_0: bool,
    /// 是否启用 F# 4.1
    pub fsharp_4_1: bool,
    /// 是否启用 F# 4.5
    pub fsharp_4_5: bool,
    /// 是否启用 F# 5.0
    pub fsharp_5_0: bool,
    /// 是否启用 F# 6.0
    pub fsharp_6_0: bool,
    /// 是否启用 F# 7.0
    pub fsharp_7_0: bool,
    /// 是否启用计算表达
    pub computation_expressions: bool,
    /// 是否启用类型提供
    pub type_providers: bool,
    /// 是否启用异步工作
    pub async_workflows: bool,
    /// 是否启用查询表达    
    pub query_expressions: bool,
}

impl Default for FSharpLanguage {
    fn default() -> Self {
        Self { fsharp_4_0: true, fsharp_4_1: true, fsharp_4_5: true, fsharp_5_0: true, fsharp_6_0: true, fsharp_7_0: true, computation_expressions: true, type_providers: true, async_workflows: true, query_expressions: true }
    }
}

impl FSharpLanguage {
    /// 创建新的 F# 语言配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 启用所F#
    pub fn with_all_features(mut self) -> Self {
        self.fsharp_4_0 = true;
        self.fsharp_4_1 = true;
        self.fsharp_4_5 = true;
        self.fsharp_5_0 = true;
        self.fsharp_6_0 = true;
        self.fsharp_7_0 = true;
        self.computation_expressions = true;
        self.type_providers = true;
        self.async_workflows = true;
        self.query_expressions = true;
        self
    }

    /// 设置 F# 版本
    pub fn with_version(mut self, major: u8, minor: u8) -> Self {
        match (major, minor) {
            (4, 0) => {
                self.fsharp_4_0 = true;
            }
            (4, 1) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
            }
            (4, 5) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
            }
            (5, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true;
            }
            (6, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true;
                self.fsharp_6_0 = true;
            }
            (7, 0) => {
                self.fsharp_4_0 = true;
                self.fsharp_4_1 = true;
                self.fsharp_4_5 = true;
                self.fsharp_5_0 = true;
                self.fsharp_6_0 = true;
                self.fsharp_7_0 = true;
            }
            _ => {}
        }
        self
    }

    /// 启用计算表达
    pub fn with_computation_expressions(mut self, enabled: bool) -> Self {
        self.computation_expressions = enabled;
        self
    }

    /// 启用类型提供
    pub fn with_type_providers(mut self, enabled: bool) -> Self {
        self.type_providers = enabled;
        self
    }

    /// 启用异步工作
    pub fn with_async_workflows(mut self, enabled: bool) -> Self {
        self.async_workflows = enabled;
        self
    }

    /// 启用查询表达
    pub fn with_query_expressions(mut self, enabled: bool) -> Self {
        self.query_expressions = enabled;
        self
    }
}

pub struct FSharpRoot;

impl Language for FSharpLanguage {
    const NAME: &'static str = "fsharp";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::FSharpSyntaxKind;
    type ElementType = crate::kind::FSharpSyntaxKind;
    type TypedRoot = FSharpRoot;
}
