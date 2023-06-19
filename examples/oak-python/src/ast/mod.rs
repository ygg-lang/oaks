use crate::kind::PythonSyntaxKind;

/// Python 源文件的根节点
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub statements: Vec<Element>,
}

/// Python 语法元素
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    /// 函数定义
    FunctionDef { name: String, args: Vec<String>, body: Vec<Element> },
    /// 类定义
    ClassDef { name: String, bases: Vec<String>, body: Vec<Element> },
    /// 赋值语句
    Assign { target: String, value: String },
    /// 表达式语句
    Expr { value: String },
    /// 导入语句
    Import { module: String, alias: Option<String> },
    /// 从模块导入
    ImportFrom { module: String, names: Vec<String> },
    /// 返回语句
    Return { value: Option<String> },
    /// 条件语句
    If { test: String, body: Vec<Element>, orelse: Vec<Element> },
    /// 循环语句
    For { target: String, iter: String, body: Vec<Element> },
    /// While 循环
    While { test: String, body: Vec<Element> },
    /// 注释
    Comment { text: String },
    /// 其他元素
    Other { kind: PythonSyntaxKind, text: String },
}
