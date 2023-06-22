use crate::{Language, ParserState, source::Source, tree::GreenNode};

/// Associativity of an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    /// Left-associative (e.g., a + b + c is (a + b) + c).
    Left,
    /// Right-associative (e.g., a = b = c is a = (b = c)).
    Right,
    /// Non-associative.
    None,
}

/// Precedence and associativity of an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorInfo {
    /// Precedence of the operator.
    pub precedence: u8,
    /// Associativity of the operator.
    pub associativity: Associativity,
}

impl OperatorInfo {
    /// Creates a left-associative operator info.
    pub fn left(precedence: u8) -> Self {
        Self { precedence, associativity: Associativity::Left }
    }

    /// Creates a right-associative operator info.
    pub fn right(precedence: u8) -> Self {
        Self { precedence, associativity: Associativity::Right }
    }

    /// Creates a non-associative operator info.
    pub fn none(precedence: u8) -> Self {
        Self { precedence, associativity: Associativity::None }
    }
}

/// A specification for a Pratt parser.
///
/// Users implement this trait to define the grammar rules for expressions.
/// Using a trait allows the compiler to optimize operator lookups using match statements.
pub trait Pratt<L: Language> {
    /// Parses a primary expression (e.g., literals, identifiers, group).
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, L, S>) -> &'a GreenNode<'a, L>;

    /// Handles prefix operators and primary expressions.
    ///
    /// Default implementation just calls `primary`.
    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, L, S>) -> &'a GreenNode<'a, L> {
        self.primary(state)
    }

    /// Handles infix and postfix operators.
    ///
    /// Should return `Some(new_node)` if an operator was parsed, or `None` if no operator matches
    /// or its precedence is lower than `min_precedence`.
    fn infix<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, L, S>, left: &'a GreenNode<'a, L>, min_precedence: u8) -> Option<&'a GreenNode<'a, L>>;
}

/// A helper for parsing binary (infix) expressions.
#[inline(always)]
pub fn binary<'a, L, S, F>(state: &mut ParserState<'a, L, S>, left: &'a GreenNode<'a, L>, op_kind: L::TokenType, op_precedence: u8, assoc: Associativity, result_kind: L::ElementType, mut parse_expr: F) -> &'a GreenNode<'a, L>
where
    L: Language,
    S: Source + ?Sized,
    F: FnMut(&mut ParserState<'a, L, S>, u8) -> &'a GreenNode<'a, L>,
    L::ElementType: From<L::TokenType>,
{
    let cp = state.checkpoint();
    state.push_child(left);
    state.expect(op_kind).ok();

    let next_prec = match assoc {
        Associativity::Left => op_precedence + 1,
        Associativity::Right => op_precedence,
        Associativity::None => op_precedence + 1,
    };

    let right = parse_expr(state, next_prec);
    state.push_child(right);
    state.finish_at(cp, result_kind)
}

/// A helper for parsing unary (prefix) expressions.
#[inline(always)]
pub fn unary<'a, L, S, F>(state: &mut ParserState<'a, L, S>, op_kind: L::TokenType, op_precedence: u8, result_kind: L::ElementType, mut parse_expr: F) -> &'a GreenNode<'a, L>
where
    L: Language,
    S: Source + ?Sized,
    F: FnMut(&mut ParserState<'a, L, S>, u8) -> &'a GreenNode<'a, L>,
    L::ElementType: From<L::TokenType>,
{
    let cp = state.checkpoint();
    state.expect(op_kind).ok();
    let right = parse_expr(state, op_precedence);
    state.push_child(right);
    state.finish_at(cp, result_kind)
}

/// A helper for parsing postfix expressions.
#[inline(always)]
pub fn postfix<'a, L, S>(state: &mut ParserState<'a, L, S>, left: &'a GreenNode<'a, L>, op_kind: L::TokenType, result_kind: L::ElementType) -> &'a GreenNode<'a, L>
where
    L: Language,
    S: Source + ?Sized,
    L::ElementType: From<L::TokenType>,
{
    let cp = state.checkpoint();
    state.push_child(left);
    state.expect(op_kind).ok();
    state.finish_at(cp, result_kind)
}

/// A Pratt parser implementation.
pub struct PrattParser<L: Language, T: Pratt<L>> {
    spec: T,
    _marker: core::marker::PhantomData<L>,
}

impl<L: Language, T: Pratt<L>> PrattParser<L, T> {
    /// Creates a new Pratt parser with the given specification.
    pub const fn new(spec: T) -> Self {
        Self { spec, _marker: core::marker::PhantomData }
    }

    /// Parses an expression with the given minimum precedence.
    pub fn parse_expr<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, L, S>, min_precedence: u8) -> &'a GreenNode<'a, L> {
        Self::parse(state, min_precedence, &self.spec)
    }

    /// Static version of parse_expr that takes a specification reference.
    pub fn parse<'a, S: Source + ?Sized>(state: &mut ParserState<'a, L, S>, min_precedence: u8, spec: &T) -> &'a GreenNode<'a, L> {
        let mut left = spec.prefix(state);

        while let Some(node) = spec.infix(state, left, min_precedence) {
            left = node;
        }

        left
    }
}
