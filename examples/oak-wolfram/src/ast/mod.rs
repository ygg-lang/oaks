#![doc = include_str!("readme.md")]
use crate::{language::WolframLanguage, parser::element_type::WolframElementType};
use oak_core::tree::{GreenNode, RedNode, TypedNode};

/// Wolfram root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframRoot<'a> {
    /// The underlying green node.
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

/// Wolfram function call (e.g., f[x, y]).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframCall<'a> {
    /// The underlying green node.
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

/// Wolfram symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframSymbol<'a> {
    /// The underlying green node.
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

/// Wolfram literal (Integer, Real, String).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframLiteral<'a> {
    /// The underlying green node.
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

/// Wolfram list {a, b, c}.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframList<'a> {
    /// The underlying green node.
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

/// Wolfram argument list [x, y].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframArguments<'a> {
    /// The underlying green node.
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

/// Wolfram binary expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframBinaryExpr<'a> {
    /// The underlying green node.
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

/// Wolfram prefix expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframPrefixExpr<'a> {
    /// The underlying green node.
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

/// Wolfram postfix expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframPostfixExpr<'a> {
    /// The underlying green node.
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

/// Wolfram parenthesized expression (expr).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WolframExpression<'a> {
    /// The underlying green node.
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
