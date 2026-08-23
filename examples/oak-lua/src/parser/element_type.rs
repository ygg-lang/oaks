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
                match token {
            crate::lexer::token_type::LuaTokenType::Root => Self::Root,
            crate::lexer::token_type::LuaTokenType::And => Self::And,
            crate::lexer::token_type::LuaTokenType::Break => Self::Break,
            crate::lexer::token_type::LuaTokenType::Do => Self::Do,
            crate::lexer::token_type::LuaTokenType::Else => Self::Else,
            crate::lexer::token_type::LuaTokenType::Elseif => Self::Elseif,
            crate::lexer::token_type::LuaTokenType::End => Self::End,
            crate::lexer::token_type::LuaTokenType::False => Self::False,
            crate::lexer::token_type::LuaTokenType::For => Self::For,
            crate::lexer::token_type::LuaTokenType::Function => Self::Function,
            crate::lexer::token_type::LuaTokenType::Goto => Self::Goto,
            crate::lexer::token_type::LuaTokenType::If => Self::If,
            crate::lexer::token_type::LuaTokenType::In => Self::In,
            crate::lexer::token_type::LuaTokenType::Local => Self::Local,
            crate::lexer::token_type::LuaTokenType::Nil => Self::Nil,
            crate::lexer::token_type::LuaTokenType::Not => Self::Not,
            crate::lexer::token_type::LuaTokenType::Or => Self::Or,
            crate::lexer::token_type::LuaTokenType::Repeat => Self::Repeat,
            crate::lexer::token_type::LuaTokenType::Return => Self::Return,
            crate::lexer::token_type::LuaTokenType::Then => Self::Then,
            crate::lexer::token_type::LuaTokenType::True => Self::True,
            crate::lexer::token_type::LuaTokenType::Until => Self::Until,
            crate::lexer::token_type::LuaTokenType::While => Self::While,
            crate::lexer::token_type::LuaTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::LuaTokenType::Number => Self::Number,
            crate::lexer::token_type::LuaTokenType::String => Self::String,
            crate::lexer::token_type::LuaTokenType::Plus => Self::Plus,
            crate::lexer::token_type::LuaTokenType::Minus => Self::Minus,
            crate::lexer::token_type::LuaTokenType::Star => Self::Star,
            crate::lexer::token_type::LuaTokenType::Slash => Self::Slash,
            crate::lexer::token_type::LuaTokenType::Percent => Self::Percent,
            crate::lexer::token_type::LuaTokenType::Caret => Self::Caret,
            crate::lexer::token_type::LuaTokenType::Hash => Self::Hash,
            crate::lexer::token_type::LuaTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::LuaTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::LuaTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::LuaTokenType::LtLt => Self::LtLt,
            crate::lexer::token_type::LuaTokenType::GtGt => Self::GtGt,
            crate::lexer::token_type::LuaTokenType::SlashSlash => Self::SlashSlash,
            crate::lexer::token_type::LuaTokenType::EqEq => Self::EqEq,
            crate::lexer::token_type::LuaTokenType::TildeEq => Self::TildeEq,
            crate::lexer::token_type::LuaTokenType::LtEq => Self::LtEq,
            crate::lexer::token_type::LuaTokenType::GtEq => Self::GtEq,
            crate::lexer::token_type::LuaTokenType::Lt => Self::Lt,
            crate::lexer::token_type::LuaTokenType::Gt => Self::Gt,
            crate::lexer::token_type::LuaTokenType::Eq => Self::Eq,
            crate::lexer::token_type::LuaTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::LuaTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::LuaTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::LuaTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::LuaTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::LuaTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::LuaTokenType::ColonColon => Self::ColonColon,
            crate::lexer::token_type::LuaTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::LuaTokenType::Colon => Self::Colon,
            crate::lexer::token_type::LuaTokenType::Comma => Self::Comma,
            crate::lexer::token_type::LuaTokenType::Dot => Self::Dot,
            crate::lexer::token_type::LuaTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::LuaTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::LuaTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::LuaTokenType::Newline => Self::Newline,
            crate::lexer::token_type::LuaTokenType::Comment => Self::Comment,
            crate::lexer::token_type::LuaTokenType::EndOfStream => Self::EndOfStream,
            crate::lexer::token_type::LuaTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
