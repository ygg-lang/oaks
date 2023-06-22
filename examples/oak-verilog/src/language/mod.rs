use oak_core::language::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VerilogLanguage {}

impl VerilogLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

// 定义 Verilog 的根节点类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogRoot {
    // 这里可以包含 Verilog 模块的顶层结构
    // 暂时使用简单的占位符
}

impl Language for VerilogLanguage {
    const NAME: &'static str = "verilog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::VerilogKind;
    type ElementType = crate::kind::VerilogKind;
    type TypedRoot = ();
}
