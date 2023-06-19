use crate::kind::VerilogKind;
use oak_core::language::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerilogLanguage;

// 定义 Verilog 的根节点类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogRoot {
    // 这里可以包含 Verilog 模块的顶层结构
    // 暂时使用简单的占位符
}

impl Language for VerilogLanguage {
    type SyntaxKind = VerilogKind;
    type TypedRoot = VerilogRoot;
}
