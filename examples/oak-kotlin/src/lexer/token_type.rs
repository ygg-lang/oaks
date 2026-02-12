use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token for the Kotlin language.
pub type KotlinToken = Token<KotlinTokenType>;

impl TokenType for KotlinTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Kotlin lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum KotlinTokenType {
    /// The root of the parse tree.
    Root,
    /// A source file node.
    SourceFile,
    /// End of stream token.
    EndOfStream,

    /// A function declaration.
    FunctionDeclaration,
    /// A class declaration.
    ClassDeclaration,
    /// A variable declaration.
    VariableDeclaration,
    /// An `if` statement.
    IfStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `return` statement.
    ReturnStatement,
    /// A block of code.
    Block,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// An assignment expression.
    AssignmentExpression,
    /// A function call expression.
    CallExpression,
    /// A member access expression.
    MemberAccessExpression,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A function parameter.
    Parameter,
    /// A type reference.
    TypeReference,

    // Keywords
    /// The `class` keyword.
    Class,
    /// The `fun` keyword.
    Fun,
    /// The `val` keyword.
    Val,
    /// The `var` keyword.
    Var,
    /// The `if` keyword.
    If,
    /// The `else` keyword.
    Else,
    /// The `when` keyword.
    When,
    /// The `for` keyword.
    For,
    /// The `while` keyword.
    While,
    /// The `return` keyword.
    Return,
    /// The `break` keyword.
    Break,
    /// The `continue` keyword.
    Continue,
    /// The `try` keyword.
    Try,
    /// The `catch` keyword.
    Catch,
    /// The `finally` keyword.
    Finally,
    /// The `throw` keyword.
    Throw,
    /// The `import` keyword.
    Import,
    /// The `package` keyword.
    Package,
    /// The `public` keyword.
    Public,
    /// The `private` keyword.
    Private,
    /// The `protected` keyword.
    Protected,
    /// The `internal` keyword.
    Internal,
    /// The `abstract` keyword.
    Abstract,
    /// The `final` keyword.
    Final,
    /// The `open` keyword.
    Open,
    /// The `override` keyword.
    Override,
    /// The `companion` keyword.
    Companion,
    /// The `object` keyword.
    Object,
    /// The `interface` keyword.
    Interface,
    /// The `enum` keyword.
    Enum,
    /// The `data` keyword.
    Data,
    /// The `sealed` keyword.
    Sealed,
    /// The `inline` keyword.
    Inline,
    /// The `suspend` keyword.
    Suspend,
    /// The `operator` keyword.
    Operator,
    /// The `infix` keyword.
    Infix,
    /// The `tailrec` keyword.
    Tailrec,
    /// The `external` keyword.
    External,
    /// The `annotation` keyword.
    Annotation,
    /// The `crossinline` keyword.
    Crossinline,
    /// The `noinline` keyword.
    Noinline,
    /// The `reified` keyword.
    Reified,
    /// The `vararg` keyword.
    Vararg,
    /// The `out` keyword.
    Out,
    /// The `in` keyword.
    In,
    /// The `is` keyword.
    Is,
    /// The `as` keyword.
    As,
    /// The `this` keyword.
    This,
    /// The `super` keyword.
    Super,
    /// The `null` keyword.
    Null,
    /// The `true` keyword.
    True,
    /// The `false` keyword.
    False,

    // Identifiers and literals
    /// An identifier.
    Identifier,
    /// A keyword (generic).
    Keyword,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A numeric literal (generic).
    NumberLiteral,
    /// An integer literal.
    IntLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A boolean literal.
    BooleanLiteral,

    // Operators
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `==` operator (generic).
    Equals,
    /// The `<` operator (generic).
    Less,
    /// The `>` operator (generic).
    Greater,
    /// The `&` operator.
    Ampersand,
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `~` operator.
    Tilde,
    /// The `!` operator.
    Exclamation,
    /// The `=` assignment operator.
    Assign,
    /// The `+=` assignment operator.
    PlusAssign,
    /// The `-=` assignment operator.
    MinusAssign,
    /// The `*=` assignment operator.
    StarAssign,
    /// The `/=` assignment operator.
    SlashAssign,
    /// The `%=` assignment operator.
    PercentAssign,
    /// The `==` equality operator.
    EqEq,
    /// The `!=` inequality operator.
    NotEq,
    /// The `<` operator.
    Lt,
    /// The `>` operator.
    Gt,
    /// The `<=` operator.
    LtEq,
    /// The `>=` operator.
    GtEq,
    /// The `&&` operator.
    AndAnd,
    /// The `||` operator.
    OrOr,
    /// The `.` operator.
    Dot,
    /// The `,` operator.
    Comma,
    /// The `:` operator.
    Colon,
    /// The `;` operator.
    Semi,
    /// The `->` operator.
    Arrow,
    /// The `::` operator.
    DoubleColon,
    /// The `..` operator.
    Range,
    /// The `?` operator.
    Question,
    /// The `!!` operator.
    ExclamationExclamation,
    /// The `@` operator.
    At,

    // Punctuation
    /// Left parenthesis (`(`).
    LParen,
    /// Right parenthesis (`)`).
    RParen,
    /// Left bracket (`[`).
    LBracket,
    /// Right bracket (`]`).
    RBracket,
    /// Left brace (`{`).
    LBrace,
    /// Right brace (`}`).
    RBrace,

    // Other
    /// A comment.
    Comment,
    /// Whitespace characters.
    Whitespace,
    /// A line break.
    Newline,
    /// An error token.
    Error,
}
