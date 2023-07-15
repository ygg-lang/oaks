#![doc = include_str!("readme.md")]
use crate::{language::MatlabLanguage, parser::element_type::MatlabElementType};
use oak_core::tree::{GreenNode, RedNode, TypedNode};

/// Matlab root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabRoot<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabRoot<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Root { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab function call (e.g. `f(x, y)`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabCall<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabCall<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Call { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab symbol / identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabSymbol<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabSymbol<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Symbol { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab literal (number / string / character).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabLiteral<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabLiteral<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Literal { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab array `[a, b; c]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabArray<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabArray<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Array { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab argument list `(…)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabArguments<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabArguments<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Arguments { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab binary expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabBinaryExpr<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabBinaryExpr<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::BinaryExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab prefix expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabPrefixExpr<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabPrefixExpr<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::PrefixExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab postfix expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabPostfixExpr<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabPostfixExpr<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::PostfixExpr { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}

/// Matlab parenthesized expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatlabExpression<'a> {
    /// The underlying green node.
    green: GreenNode<'a, MatlabLanguage>,
}

impl<'a> TypedNode<'a> for MatlabExpression<'a> {
    type Language = MatlabLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if node.element_type() == MatlabElementType::Expression { Some(Self { green: node.green().clone() }) } else { None }
    }

    fn green(&self) -> &GreenNode<'a, MatlabLanguage> {
        &self.green
    }
}
