#![doc = include_str!("readme.md")]

use crate::{
    language::WolframLanguage,
    lexer::token_type::WolframTokenType,
    parser::element_type::WolframElementType,
};
use core::range::Range;
use oak_core::{
    TokenType,
    source::Source,
    tree::{GreenNode, RedLeaf, RedNode, RedTree, TypedNode},
};
use std::borrow::Cow;

/// Typed projection of a Wolfram CST node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WolframExpr<'a> {
    /// Parenthesized / grouped expression.
    Expression(WolframExpression<'a>),
    /// Function call `f[…]`.
    Call(WolframCall<'a>),
    /// Symbol / identifier (including keyword heads finished as symbols).
    Symbol(WolframSymbol<'a>),
    /// Integer / real / string literal.
    Literal(WolframLiteral<'a>),
    /// List `{…}`.
    List(WolframList<'a>),
    /// Binary / infix operator application.
    Binary(WolframBinaryExpr<'a>),
    /// Prefix operator application.
    Prefix(WolframPrefixExpr<'a>),
    /// Postfix operator application (`&`, `!`).
    Postfix(WolframPostfixExpr<'a>),
    /// Part access `expr[[…]]`.
    Part(WolframPart<'a>),
    /// Blank pattern `_` / `__` / `___`.
    Blank(WolframBlank<'a>),
    /// Named pattern `x_`.
    Pattern(WolframPattern<'a>),
    /// Parser recovery / error node.
    Error(WolframError<'a>),
}

impl<'a> WolframExpr<'a> {
    /// Cast a red node to a typed Wolfram expression.
    pub fn cast(node: RedNode<'a, WolframLanguage>) -> Option<Self> {
        match node.element_type() {
            WolframElementType::Expression => WolframExpression::cast(node).map(Self::Expression),
            WolframElementType::Call => WolframCall::cast(node).map(Self::Call),
            WolframElementType::Symbol => WolframSymbol::cast(node).map(Self::Symbol),
            WolframElementType::Literal => WolframLiteral::cast(node).map(Self::Literal),
            WolframElementType::List => WolframList::cast(node).map(Self::List),
            WolframElementType::BinaryExpr => WolframBinaryExpr::cast(node).map(Self::Binary),
            WolframElementType::PrefixExpr => WolframPrefixExpr::cast(node).map(Self::Prefix),
            WolframElementType::PostfixExpr => WolframPostfixExpr::cast(node).map(Self::Postfix),
            WolframElementType::Part => WolframPart::cast(node).map(Self::Part),
            WolframElementType::Blank => WolframBlank::cast(node).map(Self::Blank),
            WolframElementType::Pattern => WolframPattern::cast(node).map(Self::Pattern),
            WolframElementType::Error => WolframError::cast(node).map(Self::Error),
            WolframElementType::Root | WolframElementType::Arguments => None,
        }
    }

    /// Underlying red node.
    pub fn red(self) -> RedNode<'a, WolframLanguage> {
        match self {
            Self::Expression(n) => n.red(),
            Self::Call(n) => n.red(),
            Self::Symbol(n) => n.red(),
            Self::Literal(n) => n.red(),
            Self::List(n) => n.red(),
            Self::Binary(n) => n.red(),
            Self::Prefix(n) => n.red(),
            Self::Postfix(n) => n.red(),
            Self::Part(n) => n.red(),
            Self::Blank(n) => n.red(),
            Self::Pattern(n) => n.red(),
            Self::Error(n) => n.red(),
        }
    }

    /// Absolute source span.
    pub fn span(self) -> Range<usize> {
        self.red().span()
    }

    /// Source text covered by this node.
    pub fn text<'s, S: Source + ?Sized>(self, source: &'s S) -> Cow<'s, str> {
        self.red().text(source)
    }

    /// Element kind.
    pub fn kind(self) -> WolframElementType {
        self.red().element_type()
    }
}

macro_rules! wolfram_typed_node {
    ($(#[$meta:meta])* $name:ident, $kind:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<'a> {
            red: RedNode<'a, WolframLanguage>,
        }

        impl<'a> $name<'a> {
            /// Underlying red node.
            pub fn red(self) -> RedNode<'a, WolframLanguage> {
                self.red
            }

            /// Absolute source span.
            pub fn span(self) -> Range<usize> {
                self.red.span()
            }

            /// Source text covered by this node.
            pub fn text<'s, S: Source + ?Sized>(self, source: &'s S) -> Cow<'s, str> {
                self.red.text(source)
            }
        }

        impl<'a> TypedNode<'a> for $name<'a> {
            type Language = WolframLanguage;

            fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
                if node.element_type() == WolframElementType::$kind {
                    Some(Self { red: node })
                }
                else {
                    None
                }
            }

            fn green(&self) -> &GreenNode<'a, WolframLanguage> {
                self.red.green()
            }
        }
    };
}

wolfram_typed_node!(
    /// Root of a Wolfram source unit (sequence of expressions).
    WolframRoot,
    Root
);
wolfram_typed_node!(
    /// Parenthesized expression.
    WolframExpression,
    Expression
);
wolfram_typed_node!(
    /// Function call `f[…]` / `expr[…]`.
    WolframCall,
    Call
);
wolfram_typed_node!(
    /// Symbol node.
    WolframSymbol,
    Symbol
);
wolfram_typed_node!(
    /// Literal node.
    WolframLiteral,
    Literal
);
wolfram_typed_node!(
    /// List `{…}`.
    WolframList,
    List
);
wolfram_typed_node!(
    /// Argument list `[…]`.
    WolframArguments,
    Arguments
);
wolfram_typed_node!(
    /// Binary expression.
    WolframBinaryExpr,
    BinaryExpr
);
wolfram_typed_node!(
    /// Prefix expression.
    WolframPrefixExpr,
    PrefixExpr
);
wolfram_typed_node!(
    /// Postfix expression.
    WolframPostfixExpr,
    PostfixExpr
);
wolfram_typed_node!(
    /// Part access `expr[[…]]`.
    WolframPart,
    Part
);
wolfram_typed_node!(
    /// Blank `_` / `__` / `___`.
    WolframBlank,
    Blank
);
wolfram_typed_node!(
    /// Named pattern `x_`.
    WolframPattern,
    Pattern
);
wolfram_typed_node!(
    /// Error / recovery node.
    WolframError,
    Error
);

/// Call head: either a leading expression node or a bare head token (`f` in `f[x]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WolframCallHead<'a> {
    /// Head finished as its own expression node.
    Expr(WolframExpr<'a>),
    /// Head stored as a leaf inside the call (typical `Symbol[…]`).
    Token(RedLeaf<WolframLanguage>),
}

impl<'a> WolframRoot<'a> {
    /// Wrap a parsed green root at offset 0.
    pub fn from_green(green: &'a GreenNode<'a, WolframLanguage>) -> Option<Self> {
        Self::cast(RedNode::new(green, 0))
    }

    /// Top-level expression children (skips trivia / error leaves).
    pub fn expressions(self) -> impl Iterator<Item = WolframExpr<'a>> {
        child_exprs(self.red)
    }
}

impl<'a> WolframExpression<'a> {
    /// Inner expression (first child expression node).
    pub fn inner(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).next()
    }
}

impl<'a> WolframCall<'a> {
    /// Call head (expression or bare token).
    pub fn head(self) -> Option<WolframCallHead<'a>> {
        for child in self.red.children() {
            match child {
                RedTree::Node(n) if n.element_type() != WolframElementType::Arguments => {
                    return WolframExpr::cast(n).map(WolframCallHead::Expr);
                }
                RedTree::Leaf(leaf) if !leaf.kind().is_ignored() && is_symbol_like(leaf.kind()) => {
                    return Some(WolframCallHead::Token(leaf));
                }
                _ => {}
            }
        }
        None
    }

    /// Argument-list groups (`f[a][b]` yields two).
    pub fn argument_lists(self) -> impl Iterator<Item = WolframArguments<'a>> {
        self.red.children().filter_map(|c| c.as_node()).filter_map(WolframArguments::cast)
    }

    /// Flattened argument expressions across all argument groups.
    pub fn arguments(self) -> impl Iterator<Item = WolframExpr<'a>> {
        self.argument_lists().flat_map(|args| args.items())
    }
}

impl<'a> WolframArguments<'a> {
    /// Argument expressions.
    pub fn items(self) -> impl Iterator<Item = WolframExpr<'a>> {
        child_exprs(self.red)
    }
}

impl<'a> WolframList<'a> {
    /// List element expressions.
    pub fn elements(self) -> impl Iterator<Item = WolframExpr<'a>> {
        child_exprs(self.red)
    }
}

impl<'a> WolframBinaryExpr<'a> {
    /// Left-hand operand.
    pub fn lhs(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).next()
    }

    /// Right-hand operand.
    pub fn rhs(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).nth(1)
    }

    /// Infix operator token (first non-ignored leaf after the left operand when present).
    pub fn operator(self) -> Option<RedLeaf<WolframLanguage>> {
        operator_after_first_expr(self.red)
    }
}

impl<'a> WolframPrefixExpr<'a> {
    /// Prefix operator token.
    pub fn operator(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red)
    }

    /// Operand expression.
    pub fn operand(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).next()
    }
}

impl<'a> WolframPostfixExpr<'a> {
    /// Operand expression.
    pub fn operand(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).next()
    }

    /// Postfix operator token.
    pub fn operator(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red)
    }
}

impl<'a> WolframPart<'a> {
    /// Expression being indexed.
    pub fn expression(self) -> Option<WolframExpr<'a>> {
        self.red.children().filter_map(|c| c.as_node()).find(|n| n.element_type() != WolframElementType::Arguments).and_then(WolframExpr::cast)
    }

    /// Index argument lists.
    pub fn argument_lists(self) -> impl Iterator<Item = WolframArguments<'a>> {
        self.red.children().filter_map(|c| c.as_node()).filter_map(WolframArguments::cast)
    }

    /// Flattened index expressions.
    pub fn indices(self) -> impl Iterator<Item = WolframExpr<'a>> {
        self.argument_lists().flat_map(|args| args.items())
    }
}

impl<'a> WolframBlank<'a> {
    /// Blank kind token (`_` / `__` / `___`).
    pub fn blank_token(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red).filter(|t| {
            matches!(t.kind(), WolframTokenType::Underscore | WolframTokenType::DoubleUnderscore | WolframTokenType::TripleUnderscore)
        })
    }

    /// Optional typed head (`_Integer`).
    pub fn typed_head(self) -> Option<WolframCallHead<'a>> {
        let mut saw_blank = false;
        for child in self.red.children() {
            match child {
                RedTree::Leaf(leaf) if !leaf.kind().is_ignored() => {
                    if matches!(
                        leaf.kind(),
                        WolframTokenType::Underscore | WolframTokenType::DoubleUnderscore | WolframTokenType::TripleUnderscore
                    ) {
                        saw_blank = true;
                    }
                    else if saw_blank && is_symbol_like(leaf.kind()) {
                        return Some(WolframCallHead::Token(leaf));
                    }
                }
                RedTree::Node(n) => {
                    if let Some(expr) = WolframExpr::cast(n) {
                        return Some(WolframCallHead::Expr(expr));
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl<'a> WolframPattern<'a> {
    /// Pattern name / left operand.
    pub fn name(self) -> Option<WolframExpr<'a>> {
        child_exprs(self.red).next()
    }

    /// Underscore token (`_` / `__` / `___`).
    pub fn blank_token(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red).filter(|t| {
            matches!(t.kind(), WolframTokenType::Underscore | WolframTokenType::DoubleUnderscore | WolframTokenType::TripleUnderscore)
        })
    }
}

impl<'a> WolframSymbol<'a> {
    /// Leading identifier / keyword / slot token inside this symbol node.
    pub fn token(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red)
    }
}

impl<'a> WolframLiteral<'a> {
    /// Leading literal token inside this node.
    pub fn token(self) -> Option<RedLeaf<WolframLanguage>> {
        first_significant_token(self.red)
    }
}

fn child_exprs<'a>(node: RedNode<'a, WolframLanguage>) -> impl Iterator<Item = WolframExpr<'a>> {
    node.children().filter_map(|c| c.as_node()).filter_map(WolframExpr::cast)
}

fn first_significant_token<'a>(node: RedNode<'a, WolframLanguage>) -> Option<RedLeaf<WolframLanguage>> {
    node.children().filter_map(|c| c.as_token()).find(|t| !t.kind().is_ignored())
}

fn operator_after_first_expr<'a>(node: RedNode<'a, WolframLanguage>) -> Option<RedLeaf<WolframLanguage>> {
    let mut saw_expr = false;
    for child in node.children() {
        match child {
            RedTree::Node(n) => {
                if WolframExpr::cast(n).is_some() {
                    if saw_expr {
                        break;
                    }
                    saw_expr = true;
                }
            }
            RedTree::Leaf(leaf) if !leaf.kind().is_ignored() => {
                if saw_expr {
                    return Some(leaf);
                }
            }
            _ => {}
        }
    }
    first_significant_token(node)
}

fn is_symbol_like(kind: WolframTokenType) -> bool {
    matches!(
        kind,
        WolframTokenType::Identifier
            | WolframTokenType::Slot
            | WolframTokenType::SlotSequence
            | WolframTokenType::If
            | WolframTokenType::Then
            | WolframTokenType::Else
            | WolframTokenType::While
            | WolframTokenType::For
            | WolframTokenType::Do
            | WolframTokenType::Function
            | WolframTokenType::Module
            | WolframTokenType::Block
            | WolframTokenType::With
            | WolframTokenType::Table
            | WolframTokenType::Map
            | WolframTokenType::Apply
            | WolframTokenType::Select
            | WolframTokenType::Cases
            | WolframTokenType::Rule
            | WolframTokenType::RuleDelayed
            | WolframTokenType::Set
            | WolframTokenType::SetDelayed
            | WolframTokenType::Unset
            | WolframTokenType::Clear
            | WolframTokenType::ClearAll
            | WolframTokenType::Return
            | WolframTokenType::Break
            | WolframTokenType::Continue
            | WolframTokenType::True
            | WolframTokenType::False
            | WolframTokenType::Null
            | WolframTokenType::Export
            | WolframTokenType::Import
    )
}
