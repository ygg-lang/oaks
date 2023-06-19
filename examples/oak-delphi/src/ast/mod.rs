/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone)]
pub struct DelphiRoot {
    pub items: Vec<DelphiItem>,
}

/// Delphi 顶层项
#[derive(Debug, PartialEq, Clone)]
pub enum DelphiItem {
    Program(DelphiProgram),
    Unit(DelphiUnit),
    Statement(DelphiStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DelphiProgram {
    pub name: String,
    pub statements: Vec<DelphiStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DelphiUnit {
    pub name: String,
    pub interface_section: Vec<DelphiStatement>,
    pub implementation_section: Vec<DelphiStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DelphiStatement {
    // 简化的语句类型
    Empty,
}
