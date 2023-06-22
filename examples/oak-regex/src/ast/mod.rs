use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegexRoot {
    pub alternatives: Vec<Pattern>,
}

/// Regular expression pattern
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub alternatives: Vec<Alternative>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Alternation expression (|)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub elements: Vec<PatternElement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Pattern element
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterClass {
    pub negated: bool,
    pub ranges: Vec<CharacterRange>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Character range
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterRange {
    pub start: char,
    pub end: Option<char>, // None means single character
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Quantifier
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Quantifier {
    pub element: Box<PatternElement>,
    pub kind: QuantifierKind,
    pub greedy: bool,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Quantifier type
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

/// Group
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Group {
    pub kind: GroupKind,
    pub element: Box<PatternElement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Group type
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GroupKind {
    /// Capturing group (...) with optional number
    Capturing(Option<u32>), // Number, None means auto-numbered
    /// Non-capturing group (?:...)
    NonCapturing,
    /// Positive lookahead (?=...)
    Lookahead,
    /// Negative lookahead (?!...)
    NegativeLookahead,
    /// Positive lookbehind (?<=...)
    Lookbehind,
    /// Negative lookbehind (?<!...)
    NegativeLookbehind,
    /// Atomic group (?>...)
    Atomic,
    /// Conditional group (?(condition)...)
    Conditional(Condition),
}

/// Condition
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Group number condition
    GroupNumber(u32),
    /// Recursion condition
    Recursion,
    /// Assertion condition
    Assertion(Box<PatternElement>),
}

/// Assertion
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Assertion {
    pub kind: AssertionKind,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Assertion type
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssertionKind {
    /// Start of string ^
    Start,
    /// End of string $
    End,
    /// Word boundary \b
    WordBoundary,
    /// Non-word boundary \B
    NonWordBoundary,
}

/// Literal
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub value: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Special character
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Special {
    pub kind: SpecialKind,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Special character type
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
