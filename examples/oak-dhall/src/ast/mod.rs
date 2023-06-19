use core::range::Range;

/// DHall AST 根节点
#[derive(Debug, PartialEq, Clone)]
pub struct DHallRoot {
    pub expressions: Vec<DHallExpr>,
}

/// DHall 表达式
#[derive(Debug, PartialEq, Clone)]
pub enum DHallExpr {
    /// 标识符
    Identifier { name: String, span: Range<usize> },
    /// 字面量
    Literal { value: String, span: Range<usize> },
    /// 函数应用
    Application { func: Box<DHallExpr>, arg: Box<DHallExpr>, span: Range<usize> },
    /// Lambda 表达式
    Lambda { param: String, body: Box<DHallExpr>, span: Range<usize> },
}
