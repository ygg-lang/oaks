use oak_core::{TokenType, UniversalTokenRole};

/// Ada token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdaTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comments.
    Comment,

    /// String literals.
    StringLiteral,
    /// Character literals.
    CharacterLiteral,
    /// Number literals.
    NumberLiteral,
    /// Identifiers.
    Identifier,

    /// `abort` keyword.
    Abort,
    /// `abs` keyword.
    Abs,
    /// `abstract` keyword.
    Abstract,
    /// `accept` keyword.
    Accept,
    /// `access` keyword.
    Access,
    /// `aliased` keyword.
    Aliased,
    /// `all` keyword.
    All,
    /// `and` keyword.
    And,
    /// `array` keyword.
    Array,
    /// `at` keyword.
    At,
    /// `begin` keyword.
    Begin,
    /// `body` keyword.
    Body,
    /// `case` keyword.
    Case,
    /// `constant` keyword.
    Constant,
    /// `declare` keyword.
    Declare,
    /// `delay` keyword.
    Delay,
    /// `delta` keyword.
    Delta,
    /// `digits` keyword.
    Digits,
    /// `do` keyword.
    Do,
    /// `else` keyword.
    Else,
    /// `elsif` keyword.
    Elsif,
    /// `end` keyword.
    End,
    /// `entry` keyword.
    Entry,
    /// `exception` keyword.
    Exception,
    /// `exit` keyword.
    Exit,
    /// `for` keyword.
    For,
    /// `function` keyword.
    Function,
    /// `generic` keyword.
    Generic,
    /// `goto` keyword.
    Goto,
    /// `if` keyword.
    If,
    /// `in` keyword.
    In,
    /// `interface` keyword.
    Interface,
    /// `is` keyword.
    Is,
    /// `limited` keyword.
    Limited,
    /// `loop` keyword.
    Loop,
    /// `mod` keyword.
    Mod,
    /// `new` keyword.
    New,
    /// `not` keyword.
    Not,
    /// The `null` keyword.
    Null,
    /// The `of` keyword.
    Of,
    /// The `or` keyword.
    Or,
    /// The `others` keyword.
    Others,
    /// The `out` keyword.
    Out,
    /// The `overriding` keyword.
    Overriding,
    /// The `package` keyword.
    Package,
    /// The `pragma` keyword.
    Pragma,
    /// The `private` keyword.
    Private,
    /// The `procedure` keyword.
    Procedure,
    /// The `protected` keyword.
    Protected,
    /// The `raise` keyword.
    Raise,
    /// The `range` keyword.
    Range,
    /// The `record` keyword.
    Record,
    /// The `rem` keyword.
    Rem,
    /// The `renames` keyword.
    Renames,
    /// The `requeue` keyword.
    Requeue,
    /// The `return` keyword.
    Return,
    /// The `reverse` keyword.
    Reverse,
    /// The `select` keyword.
    Select,
    /// The `separate` keyword.
    Separate,
    /// The `some` keyword.
    Some,
    /// The `subtype` keyword.
    Subtype,
    /// The `synchronized` keyword.
    Synchronized,
    /// The `tagged` keyword.
    Tagged,
    /// The `task` keyword.
    Task,
    /// The `terminate` keyword.
    Terminate,
    /// The `then` keyword.
    Then,
    /// The `type` keyword.
    Type,
    /// The `until` keyword.
    Until,
    /// The `use` keyword.
    Use,
    /// The `when` keyword.
    When,
    /// The `while` keyword.
    While,
    /// The `with` keyword.
    With,
    /// The `xor` keyword.
    Xor,

    /// Plus (`+`).
    Plus,
    /// Minus (`-`).
    Minus,
    /// Star (`*`).
    Star,
    /// Slash (`/`).
    Slash,
    /// Ampersand (`&`).
    Ampersand,
    /// Equal (`=`).
    Eq,
    /// Not equal (`/=`).
    Ne,
    /// Less than (`<`).
    Lt,
    /// Less than or equal (`<=`).
    Le,
    /// Greater than (`>`).
    Gt,
    /// Greater than or equal (`>=`).
    Ge,
    /// Assignment (`:=`).
    Assign,
    /// Colon equal (`:=`).
    ColonEq,
    /// Arrow (`=>`).
    Arrow,
    /// Dot (`.`).
    Dot,
    /// DotDot (`..`).
    DotDot,
    /// Comma (`,`).
    Comma,
    /// Colon (`:`).
    Colon,
    /// Semicolon (`;`).
    Semicolon,
    /// Bar (`|`).
    Bar,
    /// Pipe (`|`).
    Pipe,
    /// Apostrophe (`'`).
    Apostrophe,
    /// Tick (`'`).
    Tick,
    /// Left parenthesis (`(`).
    LeftParen,
    /// Right parenthesis (`)`).
    RightParen,
    /// Box (`<>`).
    Box,
    /// Double star (`**`).
    DoubleStar,
    /// Star star (`**`).
    StarStar,
    /// Left label delimiter (`<<`).
    LtLt,
    /// Right label delimiter (`>>`).
    GtGt,
    /// Left bracket (`[`).
    LeftBracket,
    /// Right bracket (`]`).
    RightBracket,
    /// Left brace (`{`).
    LeftBrace,
    /// Right brace (`}`).
    RightBrace,

    /// End of stream.
    Eof,
    /// Error token.
    Error,
}

impl AdaTokenType {
    /// Checks if it is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abort
                | Self::Abs
                | Self::Abstract
                | Self::Accept
                | Self::Access
                | Self::Aliased
                | Self::All
                | Self::And
                | Self::Array
                | Self::At
                | Self::Begin
                | Self::Body
                | Self::Case
                | Self::Constant
                | Self::Declare
                | Self::Delay
                | Self::Delta
                | Self::Digits
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::Entry
                | Self::Exception
                | Self::Exit
                | Self::For
                | Self::Function
                | Self::Generic
                | Self::Goto
                | Self::If
                | Self::In
                | Self::Interface
                | Self::Is
                | Self::Limited
                | Self::Loop
                | Self::Mod
                | Self::New
                | Self::Not
                | Self::Null
                | Self::Of
                | Self::Or
                | Self::Others
                | Self::Out
                | Self::Overriding
                | Self::Package
                | Self::Pragma
                | Self::Private
                | Self::Procedure
                | Self::Protected
                | Self::Raise
                | Self::Range
                | Self::Record
                | Self::Rem
                | Self::Renames
                | Self::Requeue
                | Self::Return
                | Self::Reverse
                | Self::Select
                | Self::Separate
                | Self::Some
                | Self::Subtype
                | Self::Synchronized
                | Self::Tagged
                | Self::Task
                | Self::Terminate
                | Self::Then
                | Self::Type
                | Self::Until
                | Self::Use
                | Self::When
                | Self::While
                | Self::With
                | Self::Xor
        )
    }

    /// Checks if it is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier)
    }

    /// Checks if it is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::CharacterLiteral | Self::NumberLiteral)
    }
}

impl TokenType for AdaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            _ if self.is_keyword() => Keyword,
            Self::Identifier => Name,
            _ if self.is_literal() => Literal,
            Self::Comment => Comment,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Ampersand
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::Assign
            | Self::ColonEq
            | Self::Arrow
            | Self::DoubleStar
            | Self::StarStar
            | Self::LtLt
            | Self::GtGt => Operator,
            Self::Dot
            | Self::DotDot
            | Self::Comma
            | Self::Colon
            | Self::Semicolon
            | Self::Bar
            | Self::Pipe
            | Self::Apostrophe
            | Self::Tick
            | Self::LeftParen
            | Self::RightParen
            | Self::Box
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace => Punctuation,
            _ => None,
        }
    }
}
