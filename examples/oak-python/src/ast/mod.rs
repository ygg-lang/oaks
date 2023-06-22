use crate::kind::PythonSyntaxKind;
use core::range::Range;

/// Python 源文件的根节点
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonRoot {
    pub items: Vec<Element>,
}

/// Python 语法元素
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    /// 函数定义
    FunctionDef { name: String, args: Vec<String>, body: Vec<Element>, span: Range<usize> },
    /// 类定义
    ClassDef { name: String, bases: Vec<String>, body: Vec<Element>, span: Range<usize> },
    /// 赋值语句
    Assign { target: String, value: String, span: Range<usize> },
    /// 表达式语句
    Expr { value: String, span: Range<usize> },
    /// 导入语句
    Import { module: String, alias: Option<String>, span: Range<usize> },
    /// 从模块导入
    ImportFrom { module: String, names: Vec<String>, span: Range<usize> },
    /// 返回语句
    Return { value: Option<String>, span: Range<usize> },
    /// 条件语句
    If { test: String, body: Vec<Element>, orelse: Vec<Element>, span: Range<usize> },
    /// 循环语句
    For { target: String, iter: String, body: Vec<Element>, span: Range<usize> },
    /// While 循环
    While { test: String, body: Vec<Element>, span: Range<usize> },
    /// 注释
    Comment { text: String, span: Range<usize> },
    /// 其他元素
    Other { kind: PythonSyntaxKind, text: String, span: Range<usize> },
}
