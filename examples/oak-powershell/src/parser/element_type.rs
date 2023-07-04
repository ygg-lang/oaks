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
                match token {
            crate::lexer::token_type::PowerShellTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::PowerShellTokenType::Newline => Self::Newline,
            crate::lexer::token_type::PowerShellTokenType::Comment => Self::Comment,
            crate::lexer::token_type::PowerShellTokenType::Begin => Self::Begin,
            crate::lexer::token_type::PowerShellTokenType::Break => Self::Break,
            crate::lexer::token_type::PowerShellTokenType::Catch => Self::Catch,
            crate::lexer::token_type::PowerShellTokenType::Class => Self::Class,
            crate::lexer::token_type::PowerShellTokenType::Continue => Self::Continue,
            crate::lexer::token_type::PowerShellTokenType::Data => Self::Data,
            crate::lexer::token_type::PowerShellTokenType::Define => Self::Define,
            crate::lexer::token_type::PowerShellTokenType::Do => Self::Do,
            crate::lexer::token_type::PowerShellTokenType::DynamicParam => Self::DynamicParam,
            crate::lexer::token_type::PowerShellTokenType::Else => Self::Else,
            crate::lexer::token_type::PowerShellTokenType::ElseIf => Self::ElseIf,
            crate::lexer::token_type::PowerShellTokenType::End => Self::End,
            crate::lexer::token_type::PowerShellTokenType::Exit => Self::Exit,
            crate::lexer::token_type::PowerShellTokenType::Filter => Self::Filter,
            crate::lexer::token_type::PowerShellTokenType::Finally => Self::Finally,
            crate::lexer::token_type::PowerShellTokenType::For => Self::For,
            crate::lexer::token_type::PowerShellTokenType::ForEach => Self::ForEach,
            crate::lexer::token_type::PowerShellTokenType::From => Self::From,
            crate::lexer::token_type::PowerShellTokenType::Function => Self::Function,
            crate::lexer::token_type::PowerShellTokenType::If => Self::If,
            crate::lexer::token_type::PowerShellTokenType::In => Self::In,
            crate::lexer::token_type::PowerShellTokenType::Param => Self::Param,
            crate::lexer::token_type::PowerShellTokenType::Process => Self::Process,
            crate::lexer::token_type::PowerShellTokenType::Return => Self::Return,
            crate::lexer::token_type::PowerShellTokenType::Switch => Self::Switch,
            crate::lexer::token_type::PowerShellTokenType::Throw => Self::Throw,
            crate::lexer::token_type::PowerShellTokenType::Trap => Self::Trap,
            crate::lexer::token_type::PowerShellTokenType::Try => Self::Try,
            crate::lexer::token_type::PowerShellTokenType::Until => Self::Until,
            crate::lexer::token_type::PowerShellTokenType::Using => Self::Using,
            crate::lexer::token_type::PowerShellTokenType::Var => Self::Var,
            crate::lexer::token_type::PowerShellTokenType::While => Self::While,
            crate::lexer::token_type::PowerShellTokenType::Workflow => Self::Workflow,
            crate::lexer::token_type::PowerShellTokenType::Plus => Self::Plus,
            crate::lexer::token_type::PowerShellTokenType::Minus => Self::Minus,
            crate::lexer::token_type::PowerShellTokenType::Multiply => Self::Multiply,
            crate::lexer::token_type::PowerShellTokenType::Divide => Self::Divide,
            crate::lexer::token_type::PowerShellTokenType::Modulo => Self::Modulo,
            crate::lexer::token_type::PowerShellTokenType::Equal => Self::Equal,
            crate::lexer::token_type::PowerShellTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::PowerShellTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::PowerShellTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::PowerShellTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::PowerShellTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::PowerShellTokenType::Like => Self::Like,
            crate::lexer::token_type::PowerShellTokenType::NotLike => Self::NotLike,
            crate::lexer::token_type::PowerShellTokenType::Match => Self::Match,
            crate::lexer::token_type::PowerShellTokenType::NotMatch => Self::NotMatch,
            crate::lexer::token_type::PowerShellTokenType::Contains => Self::Contains,
            crate::lexer::token_type::PowerShellTokenType::NotContains => Self::NotContains,
            crate::lexer::token_type::PowerShellTokenType::NotIn => Self::NotIn,
            crate::lexer::token_type::PowerShellTokenType::Replace => Self::Replace,
            crate::lexer::token_type::PowerShellTokenType::Split => Self::Split,
            crate::lexer::token_type::PowerShellTokenType::Join => Self::Join,
            crate::lexer::token_type::PowerShellTokenType::Is => Self::Is,
            crate::lexer::token_type::PowerShellTokenType::IsNot => Self::IsNot,
            crate::lexer::token_type::PowerShellTokenType::As => Self::As,
            crate::lexer::token_type::PowerShellTokenType::And => Self::And,
            crate::lexer::token_type::PowerShellTokenType::Or => Self::Or,
            crate::lexer::token_type::PowerShellTokenType::Xor => Self::Xor,
            crate::lexer::token_type::PowerShellTokenType::Not => Self::Not,
            crate::lexer::token_type::PowerShellTokenType::Band => Self::Band,
            crate::lexer::token_type::PowerShellTokenType::Bor => Self::Bor,
            crate::lexer::token_type::PowerShellTokenType::Bxor => Self::Bxor,
            crate::lexer::token_type::PowerShellTokenType::Bnot => Self::Bnot,
            crate::lexer::token_type::PowerShellTokenType::Shl => Self::Shl,
            crate::lexer::token_type::PowerShellTokenType::Shr => Self::Shr,
            crate::lexer::token_type::PowerShellTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::PowerShellTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::PowerShellTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::PowerShellTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::PowerShellTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::PowerShellTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::PowerShellTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::PowerShellTokenType::Comma => Self::Comma,
            crate::lexer::token_type::PowerShellTokenType::Dot => Self::Dot,
            crate::lexer::token_type::PowerShellTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::PowerShellTokenType::Colon => Self::Colon,
            crate::lexer::token_type::PowerShellTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::PowerShellTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::PowerShellTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::PowerShellTokenType::At => Self::At,
            crate::lexer::token_type::PowerShellTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::PowerShellTokenType::Question => Self::Question,
            crate::lexer::token_type::PowerShellTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::PowerShellTokenType::Backtick => Self::Backtick,
            crate::lexer::token_type::PowerShellTokenType::SingleQuote => Self::SingleQuote,
            crate::lexer::token_type::PowerShellTokenType::DoubleQuote => Self::DoubleQuote,
            crate::lexer::token_type::PowerShellTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::PowerShellTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::PowerShellTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::PowerShellTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::PowerShellTokenType::ArrayLiteral => Self::ArrayLiteral,
            crate::lexer::token_type::PowerShellTokenType::HashLiteral => Self::HashLiteral,
            crate::lexer::token_type::PowerShellTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::PowerShellTokenType::Variable => Self::Variable,
            crate::lexer::token_type::PowerShellTokenType::AutomaticVariable => Self::AutomaticVariable,
            crate::lexer::token_type::PowerShellTokenType::PreferenceVariable => Self::PreferenceVariable,
            crate::lexer::token_type::PowerShellTokenType::Root => Self::Root,
            crate::lexer::token_type::PowerShellTokenType::FunctionDef => Self::FunctionDef,
            crate::lexer::token_type::PowerShellTokenType::ClassDef => Self::ClassDef,
            crate::lexer::token_type::PowerShellTokenType::IfStatement => Self::IfStatement,
            crate::lexer::token_type::PowerShellTokenType::ForStatement => Self::ForStatement,
            crate::lexer::token_type::PowerShellTokenType::ForEachStatement => Self::ForEachStatement,
            crate::lexer::token_type::PowerShellTokenType::WhileStatement => Self::WhileStatement,
            crate::lexer::token_type::PowerShellTokenType::DoWhileStatement => Self::DoWhileStatement,
            crate::lexer::token_type::PowerShellTokenType::SwitchStatement => Self::SwitchStatement,
            crate::lexer::token_type::PowerShellTokenType::TryStatement => Self::TryStatement,
            crate::lexer::token_type::PowerShellTokenType::CatchBlock => Self::CatchBlock,
            crate::lexer::token_type::PowerShellTokenType::FinallyBlock => Self::FinallyBlock,
            crate::lexer::token_type::PowerShellTokenType::ParamBlock => Self::ParamBlock,
            crate::lexer::token_type::PowerShellTokenType::ProcessBlock => Self::ProcessBlock,
            crate::lexer::token_type::PowerShellTokenType::BeginBlock => Self::BeginBlock,
            crate::lexer::token_type::PowerShellTokenType::EndBlock => Self::EndBlock,
            crate::lexer::token_type::PowerShellTokenType::ExpressionStatement => Self::ExpressionStatement,
            crate::lexer::token_type::PowerShellTokenType::Pipeline => Self::Pipeline,
            crate::lexer::token_type::PowerShellTokenType::Command => Self::Command,
            crate::lexer::token_type::PowerShellTokenType::CommandParameter => Self::CommandParameter,
            crate::lexer::token_type::PowerShellTokenType::CommandArgument => Self::CommandArgument,
            crate::lexer::token_type::PowerShellTokenType::Error => Self::Error,
            crate::lexer::token_type::PowerShellTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
