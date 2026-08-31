use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for Lua.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum LuaElementType {
    /// The root element.
    Root,
    // Keywords
    /// The `and` keyword.
    And,
    /// The `break` keyword.
    Break,
    /// The `do` keyword.
    Do,
    /// The `else` keyword.
    Else,
    /// The `elseif` keyword.
    Elseif,
    /// The `end` keyword.
    End,
    /// The `false` keyword.
    False,
    /// The `for` keyword.
    For,
    /// The `function` keyword.
    Function,
    /// The `goto` keyword.
    Goto,
    /// The `if` keyword.
    If,
    /// The `in` keyword.
    In,
    /// The `local` keyword.
    Local,
    /// The `nil` keyword.
    Nil,
    /// The `not` keyword.
    Not,
    /// The `or` keyword.
    Or,
    /// The `repeat` keyword.
    Repeat,
    /// The `return` keyword.
    Return,
    /// The `then` keyword.
    Then,
    /// The `true` keyword.
    True,
    /// The `until` keyword.
    Until,
    /// The `while` keyword.
    While,

    /// An identifier.
    Identifier,
    /// A numeric literal.
    Number,
    /// A string literal.
    String,

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
    /// The `^` operator.
    Caret,
    /// The `#` operator.
    Hash,
    /// The `&` operator.
    Ampersand,
    /// The `~` operator.
    Tilde,
    /// The `|` operator.
    Pipe,
    /// The `<<` operator.
    LtLt,
    /// The `>>` operator.
    GtGt,
    /// The `//` operator.
    SlashSlash,
    /// The `==` operator.
    EqEq,
    /// The `~=` operator.
    TildeEq,
    /// The `<=` operator.
    LtEq,
    /// The `>=` operator.
    GtEq,
    /// The `<` operator.
    Lt,
    /// The `>` operator.
    Gt,
    /// The `=` operator.
    Eq,

    /// The `(` punctuation.
    LeftParen,
    /// The `)` punctuation.
    RightParen,
    /// The `{` punctuation.
    LeftBrace,
    /// The `}` punctuation.
    RightBrace,
    /// The `[` punctuation.
    LeftBracket,
    /// The `]` punctuation.
    RightBracket,
    /// The `::` punctuation.
    ColonColon,
    /// The `;` punctuation.
    Semicolon,
    /// The `:` punctuation.
    Colon,
    /// The `,` punctuation.
    Comma,
    /// The `.` punctuation.
    Dot,
    /// The `..` punctuation.
    DotDot,
    /// The `...` punctuation.
    DotDotDot,

    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// A comment.
    Comment,

    /// End of stream marker.
    EndOfStream,
    /// Error marker.
    Error,

    /// A source file.
    SourceFile,
    /// A function declaration.
    FunctionDeclaration,
    /// A parameter list.
    ParameterList,
    /// A parameter.
    Parameter,
    /// A block statement.
    BlockStatement,
    /// A local statement.
    LocalStatement,
    /// An assignment statement.
    AssignmentStatement,
    /// An expression statement.
    ExpressionStatement,
    /// An if statement.
    IfStatement,
    /// A while statement.
    WhileStatement,
    /// A for statement.
    ForStatement,
    /// A repeat statement.
    RepeatStatement,
    /// A do statement.
    DoStatement,
    /// A break statement.
    BreakStatement,
    /// A return statement.
    ReturnStatement,
    /// A goto statement.
    GotoStatement,
    /// A label statement.
    LabelStatement,
    /// An identifier expression.
    IdentifierExpression,
    /// A literal expression.
    LiteralExpression,
    /// A boolean literal.
    BooleanLiteral,
    /// A nil literal.
    NilLiteral,
    /// A parenthesized expression.
    ParenthesizedExpression,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// A call expression.
    CallExpression,
    /// A member expression.
    MemberExpression,
    /// An index expression.
    IndexExpression,
    /// A table constructor expression.
    TableConstructorExpression,
    /// A function expression.
    FunctionExpression,
    /// A vararg expression.
    VarargExpression,
    /// A table field.
    TableField,
    /// A field.
    Field,
    /// A field list.
    FieldList,
    /// Argument list.
    ArgumentList,
    /// Variable list.
    VariableList,
    /// Expression list.
    ExpressionList,
    /// Name list.
    NameList,
    /// Function name.
    FunctionName,
    /// Function body.
    FunctionBody,
    /// Chunk statement.
    ChunkStatement,
    /// Statement list.
    StatementList,
}

impl ElementType for LuaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::LuaTokenType> for LuaElementType {
    fn from(token: crate::lexer::token_type::LuaTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
