use crate::{
    Language,
    parser::ParserState,
    source::Source,
    tree::{GreenBuilder, GreenNode, GreenTree},
};
use core::marker::PhantomData;
use triomphe::Arc;

/// Pratt parser: supports prefix/infix/postfix (postfix left for language layer special handling)
pub struct PrattParser<L: Language> {
    token: PhantomData<L::SyntaxKind>,
    /// unary prefix
    prefix_ops: Vec<PrefixEntry<L::SyntaxKind>>,
    /// binary infix
    infix_ops: Vec<InfixEntry<L::SyntaxKind>>,
}

/// Operator precedence
pub type Precedence = u8;

/// Operator associativity defines how operators of the same precedence are grouped.
///
/// Associativity determines whether an expression like `a + b + c` is interpreted as
/// `(a + b) + c` (left-associative) or `a + (b + c)` (right-associative).
/// Non-associative operators cannot be chained together without explicit parentheses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    /// Left-associative operators group from left to right.
    /// For example, subtraction is left-associative: `a - b - c` means `(a - b) - c`
    Left,
    /// Right-associative operators group from right to left.
    /// For example, assignment is right-associative: `a = b = c` means `a = (b = c)`
    Right,
    /// Non-associative operators cannot be chained without parentheses.
    /// For example, comparison operators are typically non-associative
    None,
}

/// Operator information containing precedence and associativity.
///
/// This structure encapsulates the essential properties needed for operator
/// precedence parsing, including the precedence level and how operators of the
/// same precedence should be grouped.
#[derive(Debug, Clone)]
pub struct OperatorInfo {
    /// The precedence level of the operator (higher values have higher precedence)
    pub precedence: Precedence,
    /// The associativity of the operator (Left, Right, or None)
    pub associativity: Associativity,
}

impl OperatorInfo {
    /// Creates a new operator with the specified precedence and associativity.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator (higher values have higher precedence)
    /// * `associativity` - The associativity of the operator (Left, Right, or None)
    ///
    /// # Examples
    ///
    /// ```
    /// let op = OperatorInfo::new(10, Associativity::Left);
    /// ```
    pub fn new(precedence: Precedence, associativity: Associativity) -> Self {
        Self { precedence, associativity }
    }

    /// Creates a left-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let left_op = OperatorInfo::left(5);
    /// ```
    pub fn left(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::Left)
    }

    /// Creates a right-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let right_op = OperatorInfo::right(8);
    /// ```
    pub fn right(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::Right)
    }

    /// Creates a non-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let non_assoc_op = OperatorInfo::none(3);
    /// ```
    pub fn none(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::None)
    }
}

#[derive(Clone)]
struct PrefixEntry<K> {
    op: K,
    info: OperatorInfo,
    node_kind: K,
}

#[derive(Clone)]
struct InfixEntry<K> {
    op: K,
    info: OperatorInfo,
    node_kind: K,
}

impl<L: Language> PrattParser<L>
where
    L::SyntaxKind: Copy + PartialEq,
{
    /// Creates a new Pratt parser
    pub fn new() -> Self {
        Self { token: PhantomData, prefix_ops: Vec::new(), infix_ops: Vec::new() }
    }

    /// Registers a prefix operator
    pub fn prefix(&mut self, op: L::SyntaxKind, info: OperatorInfo, node_kind: L::SyntaxKind) -> &mut Self {
        self.prefix_ops.push(PrefixEntry { op, info, node_kind });
        self
    }

    /// Registers an infix operator
    pub fn infix(&mut self, op: L::SyntaxKind, info: OperatorInfo, node_kind: L::SyntaxKind) -> &mut Self {
        self.infix_ops.push(InfixEntry { op, info, node_kind });
        self
    }

    fn find_prefix(&self, k: L::SyntaxKind) -> Option<&PrefixEntry<L::SyntaxKind>> {
        self.prefix_ops.iter().find(|e| e.op == k)
    }

    fn find_infix(&self, k: L::SyntaxKind) -> Option<&InfixEntry<L::SyntaxKind>> {
        self.infix_ops.iter().find(|e| e.op == k)
    }

    /// Parses expressions using the Pratt algorithm
    /// - `min_bp`: Minimum binding strength (precedence)
    /// - `primary`: Primary expression parser provided by language layer (includes parentheses, literals, identifiers, and high-binding suffixes like calls/fields)
    pub fn parse<S, F>(
        &self,
        state: &mut ParserState<'_, S, L>,
        min_bp: Precedence,
        primary: &F,
    ) -> Arc<GreenNode<L::SyntaxKind>>
    where
        S: Source,
        F: Fn(&mut ParserState<'_, S, L>) -> Arc<GreenNode<L::SyntaxKind>>,
    {
        // Handle prefix or primary expression
        let mut left = if let Some(k) = state.peek_kind() {
            if let Some(prefix) = self.find_prefix(k) {
                // consume op and copy needed fields immediately to drop borrow
                let (op_kind, op_len) = match state.advance() {
                    Some(t) => (t.kind, t.span.end - t.span.start),
                    None => return primary(state),
                };
                let rhs = self.parse(state, prefix.info.precedence, primary);
                let mut builder = GreenBuilder::<L>::new(128);
                builder = builder.token(op_kind, op_len);
                builder = builder.push(GreenTree::Node(rhs));
                builder.finish(prefix.node_kind)
            }
            else {
                primary(state)
            }
        }
        else {
            primary(state)
        };

        // Handle infix chain
        loop {
            if !state.not_at_end() {
                break;
            }
            let op_kind = match state.peek_kind() {
                Some(k) => k,
                None => break,
            };
            let infix = match self.find_infix(op_kind) {
                Some(i) => i,
                None => break,
            };

            let p = infix.info.precedence;
            let lbp = p;
            let rbp = match infix.info.associativity {
                Associativity::Left | Associativity::None => p + 1,
                Associativity::Right => p,
            };

            if lbp < min_bp {
                break;
            }

            // consume op and copy needed fields immediately to drop borrow
            let (op_kind2, op_len2) = match state.advance() {
                Some(t) => (t.kind, t.span.end - t.span.start),
                None => break,
            };
            let right = self.parse(state, rbp, primary);

            let mut builder = GreenBuilder::<L>::new(128);
            builder = builder.push(GreenTree::Node(left));
            builder = builder.token(op_kind2, op_len2);
            builder = builder.push(GreenTree::Node(right));
            left = builder.finish(infix.node_kind);
        }

        left
    }
}
