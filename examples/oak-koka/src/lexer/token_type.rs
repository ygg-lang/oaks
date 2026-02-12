use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token for the Koka language.
pub type KokaToken = Token<KokaTokenType>;

impl TokenType for KokaTokenType {
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

/// Token types for the Koka lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum KokaTokenType {
    /// The root of the parse tree.
    Root,
    /// A source file node.
    SourceFile,
    /// End of stream token.
    EndOfStream,

    // Declarations
    /// A function declaration.
    FunctionDeclaration,
    /// A class declaration.
    ClassDeclaration,
    /// An effect declaration.
    EffectDeclaration,
    /// A handler declaration.
    HandlerDeclaration,
    /// A type declaration.
    TypeDeclaration,
    /// An alias declaration.
    AliasDeclaration,
    /// A struct declaration.
    StructDeclaration,
    /// A variable declaration.
    VariableDeclaration,
    /// A module declaration.
    ModuleDeclaration,

    // Statements and Expressions
    /// An `if` statement.
    IfStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `match` statement.
    MatchStatement,
    /// A `with` statement (for handlers).
    WithStatement,
    /// A `return` statement.
    ReturnStatement,
    /// A block of code.
    Block,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// An assignment expression (`:=`).
    AssignmentExpression,
    /// A function call expression.
    CallExpression,
    /// A member access expression (`.`).
    MemberAccessExpression,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A function parameter.
    Parameter,
    /// A type reference.
    TypeReference,
    /// An effect reference (e.g., `<console>`).
    EffectReference,

    // Keywords
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
    /// The `match` keyword.
    Match,
    /// The `with` keyword.
    With,
    /// The `handler` keyword.
    Handler,
    /// The `effect` keyword.
    Effect,
    /// The `control` keyword.
    Control,
    /// The `resume` keyword.
    Resume,
    /// The `return` keyword.
    Return,
    /// The `import` keyword.
    Import,
    /// The `module` keyword.
    Module,
    /// The `pub` keyword.
    Pub,
    /// The `alias` keyword.
    Alias,
    /// The `struct` keyword.
    Struct,
    /// The `type` keyword.
    Type,
    /// The `class` keyword.
    Class,
    /// The `while` keyword.
    While,
    /// The `forall` keyword.
    Forall,
    /// The `exists` keyword.
    Exists,
    /// The `linear` keyword.
    Linear,
    /// The `fixed` keyword.
    Fixed,

    // Identifiers and literals
    /// An identifier.
    Identifier,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A boolean literal (`true`, `false`).
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
    /// The `==` operator.
    EqEq,
    /// The `!=` operator.
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
    /// The `!` operator.
    Exclamation,
    /// The `!!` operator.
    ExclamationExclamation,
    /// The `?` operator.
    Question,
    /// The `@` operator.
    At,
    /// The `~` operator.
    Tilde,
    /// The `=` operator.
    Assign,
    /// The `+=` operator.
    PlusAssign,
    /// The `-=` operator.
    MinusAssign,
    /// The `*=` operator.
    StarAssign,
    /// The `/=` operator.
    SlashAssign,
    /// The `%=` operator.
    PercentAssign,
    /// The `:=` assignment operator.
    ColonAssign,
    /// The `.` operator.
    Dot,
    /// The `..` operator.
    Range,
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
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `&` operator.
    Ampersand,
    /// The `is` keyword.
    Is,
    /// The `as` keyword.
    As,

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
    /// Left angle bracket (`<`).
    LAngle,
    /// Right angle bracket (`>`).
    RAngle,

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
