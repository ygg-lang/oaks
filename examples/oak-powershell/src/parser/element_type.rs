use oak_core::{ElementType, UniversalElementRole};

/// Element types for the PowerShell language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PowerShellElementType {
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// A comment.
    Comment,

    /// `begin` keyword.
    Begin,
    /// `break` keyword.
    Break,
    /// `catch` keyword.
    Catch,
    /// `class` keyword.
    Class,
    /// `continue` keyword.
    Continue,
    /// `data` keyword.
    Data,
    /// `define` keyword.
    Define,
    /// `do` keyword.
    Do,
    /// `dynamicparam` keyword.
    DynamicParam,
    /// `else` keyword.
    Else,
    /// `elseif` keyword.
    ElseIf,
    /// `end` keyword.
    End,
    /// `exit` keyword.
    Exit,
    /// `filter` keyword.
    Filter,
    /// `finally` keyword.
    Finally,
    /// `for` keyword.
    For,
    /// `foreach` keyword.
    ForEach,
    /// `from` keyword.
    From,
    /// `function` keyword.
    Function,
    /// `if` keyword.
    If,
    /// `in` keyword.
    In,
    /// `param` keyword.
    Param,
    /// `process` keyword.
    Process,
    /// `return` keyword.
    Return,
    /// `switch` keyword.
    Switch,
    /// `throw` keyword.
    Throw,
    /// `trap` keyword.
    Trap,
    /// `try` keyword.
    Try,
    /// `until` keyword.
    Until,
    /// `using` keyword.
    Using,
    /// `var` keyword.
    Var,
    /// `while` keyword.
    While,
    /// `workflow` keyword.
    Workflow,

    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Multiply,
    /// `/`.
    Divide,
    /// `%`.
    Modulo,
    /// `=`.
    Equal,
    /// `!=`.
    NotEqual,
    /// `>`.
    GreaterThan,
    /// `<`.
    LessThan,
    /// `>=`.
    GreaterEqual,
    /// `<=`.
    LessEqual,
    /// `-like`.
    Like,
    /// `-notlike`.
    NotLike,
    /// `-match`.
    Match,
    /// `-notmatch`.
    NotMatch,
    /// `-contains`.
    Contains,
    /// `-notcontains`.
    NotContains,
    /// `-notin`.
    NotIn,
    /// `-replace`.
    Replace,
    /// `-split`.
    Split,
    /// `-join`.
    Join,
    /// `-is`.
    Is,
    /// `-isnot`.
    IsNot,
    /// `-as`.
    As,
    /// `-and`.
    And,
    /// `-or`.
    Or,
    /// `-xor`.
    Xor,
    /// `-not`.
    Not,
    /// `-band`.
    Band,
    /// `-bor`.
    Bor,
    /// `-bxor`.
    Bxor,
    /// `-bnot`.
    Bnot,
    /// `-shl`.
    Shl,
    /// `-shr`.
    Shr,

    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `;`.
    Semicolon,
    /// `,`.
    Comma,
    /// `.`.
    Dot,
    /// `..`.
    DotDot,
    /// `:`.
    Colon,
    /// `::`.
    DoubleColon,
    /// `|`.
    Pipe,
    /// `&`.
    Ampersand,
    /// `@`.
    At,
    /// `$`.
    Dollar,
    /// `?`.
    Question,
    /// `!`.
    Exclamation,
    /// `` ` ``.
    Backtick,
    /// `'`.
    SingleQuote,
    /// `"`.
    DoubleQuote,

    /// A string literal.
    StringLiteral,
    /// A number literal.
    NumberLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// A null literal.
    NullLiteral,
    /// An array literal.
    ArrayLiteral,
    /// A hash literal.
    HashLiteral,

    /// An identifier.
    Identifier,
    /// A variable.
    Variable,
    /// An automatic variable.
    AutomaticVariable,
    /// A preference variable.
    PreferenceVariable,

    /// Root node of the AST.
    Root,
    /// A function definition.
    FunctionDef,
    /// A class definition.
    ClassDef,
    /// An `if` statement.
    IfStatement,
    /// A `for` statement.
    ForStatement,
    /// A `foreach` statement.
    ForEachStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `do-while` statement.
    DoWhileStatement,
    /// A `switch` statement.
    SwitchStatement,
    /// A `try` statement.
    TryStatement,
    /// A `catch` block.
    CatchBlock,
    /// A `finally` block.
    FinallyBlock,
    /// A `param` block.
    ParamBlock,
    /// A `process` block.
    ProcessBlock,
    /// A `begin` block.
    BeginBlock,
    /// An `end` block.
    EndBlock,
    /// An expression statement.
    ExpressionStatement,
    /// A pipeline.
    Pipeline,
    /// A command.
    Command,
    /// A command parameter.
    CommandParameter,
    /// A command argument.
    CommandArgument,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
}

impl ElementType for PowerShellElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PowerShellTokenType> for PowerShellElementType {
    fn from(token: crate::lexer::token_type::PowerShellTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
