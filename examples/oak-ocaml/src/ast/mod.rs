use std::{range::Range, string::String, vec::Vec};

/// OCaml AST 根节点
#[derive(Debug, PartialEq, Clone)]
pub struct OCamlRoot {
    pub items: Vec<OCamlItem>,
}

/// OCaml 顶层项
#[derive(Debug, PartialEq, Clone)]
pub enum OCamlItem {
    Expression(OCamlExpr),
}

/// OCaml 表达式
#[derive(Debug, PartialEq, Clone)]
pub enum OCamlExpr {
    Identifier { name: String, span: Range<usize> },
    Literal { value: String, span: Range<usize> },
}
