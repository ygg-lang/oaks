#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegexRoot {
    pub alternatives: Vec<Pattern>,
}

/// Regular expression pattern
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pattern {
    pub alternatives: Vec<Alternative>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Alternation expression (|)
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alternative {
    pub elements: Vec<PatternElement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Pattern element
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatternElement {
    /// Character class
    CharacterClass(CharacterClass),
    /// Quantifier
    Quantifier(Quantifier),
    /// Group
    Group(Group),
    /// Assertion
    Assertion(Assertion),
    /// Literal
    Literal(Literal),
    /// Special character
    Special(Special),
}

/// Character class
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterClass {
    /// Whether the character class is negated ([^...])
    pub negated: bool,
    /// The ranges and characters within the class
    pub ranges: Vec<CharacterRange>,
    /// The source span of the character class
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Character range
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CharacterRange {
    /// The start character of the range
    pub start: char,
    /// The end character of the range, or None if it's a single character
    pub end: Option<char>,
    /// The source span of the range
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Quantifier
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quantifier {
    /// The element being quantified
    pub element: Box<PatternElement>,
    /// The kind of quantifier (?, *, +, {n}, {n,m})
    pub kind: QuantifierKind,
    /// Whether the quantifier is greedy
    pub greedy: bool,
    /// The source span of the quantifier
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Quantifier type
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QuantifierKind {
    /// Zero or one (?)
    ZeroOrOne,
    /// Zero or more (*)
    ZeroOrMore,
    /// One or more (+)
    OneOrMore,
    /// Exact count {n}
    Exact(u32),
    /// Range count {n,m}
    Range(u32, Option<u32>), // None means no upper limit
}

/// Represents a capturing or non-capturing group in a regular expression.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Group {
    /// The type of group (e.g., capturing, non-capturing, lookahead).
    pub kind: GroupKind,
    /// The element contained within the group.
    pub element: Box<PatternElement>,
    /// The range in the source code where this group is located.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents the type of a regex group.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroupKind {
    /// A capturing group `(...)` with an optional capture group number.
    Capturing(Option<u32>),
    /// A non-capturing group `(?:...)`.
    NonCapturing,
    /// A positive lookahead assertion `(?=...)`.
    Lookahead,
    /// A negative lookahead assertion `(?!...)`.
    NegativeLookahead,
    /// A positive lookbehind assertion `(?<=...)`.
    Lookbehind,
    /// A negative lookbehind assertion `(?<!...)`.
    NegativeLookbehind,
    /// An atomic group `(?>...)`.
    Atomic,
    /// A conditional group `(?(condition)...)`.
    Conditional(Condition),
}

/// Represents a condition within a conditional group.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Condition {
    /// A condition based on whether a group number was matched.
    GroupNumber(u32),
    /// A condition based on whether the parser is currently recursing.
    Recursion,
    /// A condition based on a sub-assertion.
    Assertion(Box<PatternElement>),
}

/// Represents an assertion in a regular expression (e.g., `^`, `$`, `\b`).
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Assertion {
    /// The type of assertion.
    pub kind: AssertionKind,
    /// The range in the source code where this assertion is located.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents the type of a regex assertion.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssertionKind {
    /// The start of the string or line `^`.
    Start,
    /// The end of the string or line `$`.
    End,
    /// A word boundary assertion `\b`.
    WordBoundary,
    /// A non-word boundary assertion `\B`.
    NonWordBoundary,
}

/// Represents a literal character or escape sequence in a regular expression.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Literal {
    /// The literal value.
    pub value: String,
    /// The range in the source code where this literal is located.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a special regex character (e.g., `.`).
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Special {
    /// The type of special character.
    pub kind: SpecialKind,
    /// The range in the source code where this special character is located.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Special character type
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecialKind {
    /// Any character .
    Any,
    /// Digit \d
    Digit,
    /// Non-digit \D
    NonDigit,
    /// Word character \w
    Word,
    /// Non-word character \W
    NonWord,
    /// Whitespace \s
    Whitespace,
    /// Non-whitespace \S
    NonWhitespace,
    /// Backspace \b
    Backspace,
    /// Form feed \f
    FormFeed,
    /// Newline \n
    Newline,
    /// Carriage return \r
    CarriageReturn,
    /// Tab \t
    Tab,
    /// Vertical tab \v
    VerticalTab,
    /// Octal escape \ooo
    Octal(u32),
    /// Hexadecimal escape \xhh
    Hexadecimal(u32),
    /// Unicode escape \uhhhh or \U{hhhhh}
    Unicode(u32),
    /// Control character \cX
    Control(char),
    /// Named reference \k<name>
    NamedReference(String),
}
