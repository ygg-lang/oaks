#![doc = include_str!("readme.md")]
use crate::{language::WolframLanguage, parser::element_type::WolframElementType};
use oak_core::tree::{GreenNode, RedNode, TypedNode};

/// Wolfram 根节点
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframRoot<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframRoot<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Root { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 函数调用 (e.g., f[x, y])
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframCall<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframCall<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Call { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 符号
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframSymbol<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframSymbol<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Symbol { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 字面量 (Integer, Real, String)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframLiteral<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframLiteral<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Literal { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 列表 {a, b, c}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframList<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframList<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::List { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 参数列表 [x, y]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframArguments<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframArguments<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Arguments { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 二元表达式
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframBinaryExpr<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframBinaryExpr<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::BinaryExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 前缀表达式
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframPrefixExpr<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframPrefixExpr<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::PrefixExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 后缀表达式
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframPostfixExpr<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframPostfixExpr<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::PostfixExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}

/// Wolfram 括号表达式 (expr)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframExpression<'a> {
    green: GreenNode<'a, WolframLanguage>,
}

impl<'a> TypedNode<'a> for WolframExpression<'a> {
    type Language = WolframLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == WolframElementType::Expression { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, WolframLanguage> {
        &self.green
    }
}
