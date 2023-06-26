use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Dart language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DartElementType {
    /// Root element.
    Root,
    /// Class declaration.
    ClassDeclaration,
    /// Function declaration.
    FunctionDeclaration,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Identifier token.
    Identifier,
    /// Integer literal token.
    IntegerLiteral,
    /// Double literal token.
    DoubleLiteral,
    /// String literal token.
    StringLiteral,
    /// Boolean literal token.
    BooleanLiteral,
    /// Null literal token.
    NullLiteral,
    /// `abstract` keyword.
    Abstract,
    /// `as` keyword.
    As,
    /// `assert` keyword.
    Assert,
    /// `async` keyword.
    Async,
    /// `await` keyword.
    Await,
    /// `break` keyword.
    Break,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `class` keyword.
    Class,
    /// `const` keyword.
    Const,
    /// `continue` keyword.
    Continue,
    /// `covariant` keyword.
    Covariant,
    /// `default` keyword.
    Default,
    /// `deferred` keyword.
    Deferred,
    /// `do` keyword.
    Do,
    /// `dynamic` keyword.
    Dynamic,
    /// `else` keyword.
    Else,
    /// `enum` keyword.
    Enum,
    /// `export` keyword.
    Export,
    /// `extends` keyword.
    Extends,
    /// `extension` keyword.
    Extension,
    /// `external` keyword.
    External,
    /// `factory` keyword.
    Factory,
    /// `false` keyword.
    False,
    /// `final` keyword.
    Final,
    /// `finally` keyword.
    Finally,
    /// `for` keyword.
    For,
    /// `function` keyword.
    Function,
    /// `get` keyword.
    Get,
    /// `hide` keyword.
    Hide,
    /// `if` keyword.
    If,
    /// `implements` keyword.
    Implements,
    /// `import` keyword.
    Import,
    /// `in` keyword.
    In,
    /// `interface` keyword.
    Interface,
    /// `int` keyword.
    Int,
    /// `is` keyword.
    Is,
    /// `late` keyword.
    Late,
    /// `library` keyword.
    Library,
    /// `mixin` keyword.
    Mixin,
    /// `new` keyword.
    New,
    /// `null` keyword.
    Null,
    /// `on` keyword.
    On,
    /// `operator` keyword.
    Operator,
    /// `part` keyword.
    Part,
    /// `required` keyword.
    Required,
    /// `rethrow` keyword.
    Rethrow,
    /// `return` keyword.
    Return,
    /// `set` keyword.
    Set,
    /// `show` keyword.
    Show,
    /// `static` keyword.
    Static,
    /// `super` keyword.
    Super,
    /// `switch` keyword.
    Switch,
    /// `sync` keyword.
    Sync,
    /// `this` keyword.
    This,
    /// `throw` keyword.
    Throw,
    /// `true` keyword.
    True,
    /// `try` keyword.
    Try,
    /// `typedef` keyword.
    Typedef,
    /// `var` keyword.
    Var,
    /// `void` keyword.
    Void,
    /// `while` keyword.
    While,
    /// `with` keyword.
    With,
    /// `yield` keyword.
    Yield,
    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Star operator `*`.
    Star,
    /// Slash operator `/`.
    Slash,
    /// Percent operator `%`.
    Percent,
    /// Tilde-slash operator `~/`.
    TildeSlash,
    /// Equal operator `=`.
    Equal,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    BangEqual,
    /// Less-than operator `<`.
    Less,
    /// Greater-than operator `>`.
    Greater,
    /// Less-than-or-equal operator `<=`.
    LessEqual,
    /// Greater-than-or-equal operator `>=`.
    GreaterEqual,
    /// Left shift operator `<<`.
    LeftShift,
    /// Right shift operator `>>`.
    RightShift,
    /// Bitwise AND operator `&`.
    Ampersand,
    /// Bitwise OR operator `|`.
    Pipe,
    /// Bitwise XOR operator `^`.
    Caret,
    /// Bitwise NOT operator `~`.
    Tilde,
    /// Logical NOT operator `!`.
    Bang,
    /// Logical AND operator `&&`.
    AmpersandAmpersand,
    /// Logical OR operator `||`.
    PipePipe,
    /// Question mark operator `?`.
    Question,
    /// Null-aware operator `??`.
    QuestionQuestion,
    /// Increment operator `++`.
    PlusPlus,
    /// Decrement operator `--`.
    MinusMinus,
    /// Addition assignment operator `+=`.
    PlusEqual,
    /// Subtraction assignment operator `-=`.
    MinusEqual,
    /// Multiplication assignment operator `*=`.
    StarEqual,
    /// Division assignment operator `/=`.
    SlashEqual,
    /// Modulo assignment operator `%=`.
    PercentEqual,
    /// Integer division assignment operator `~/=`.
    TildeSlashEqual,
    /// Left shift assignment operator `<<=`.
    LeftShiftEqual,
    /// Right shift assignment operator `>>=`.
    RightShiftEqual,
    /// Bitwise AND assignment operator `&=`.
    AmpersandEqual,
    /// Bitwise OR assignment operator `|=`.
    PipeEqual,
    /// Bitwise XOR assignment operator `^=`.
    CaretEqual,
    /// Null-aware assignment operator `??=`.
    QuestionQuestionEqual,
    /// Arrow operator `=>`.
    Arrow,
    /// Dot operator `.`.
    Dot,
    /// Cascade operator `..`.
    DotDot,
    /// Spread operator `...`.
    DotDotDot,
    /// Null-aware access operator `?.`.
    QuestionDot,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,
    /// At symbol `@`.
    At,
    /// Hash symbol `#`.
    Hash,
    /// Line comment.
    LineComment,
    /// Block comment.
    BlockComment,
    /// Documentation comment.
    DocComment,
    /// Error token.
    Error,
    /// End of file token.
    Eof,
    /// Variable declaration.
    VariableDeclaration,
}

/// Implementation of ElementType trait for DartElementType.
impl ElementType for DartElementType {
    type Role = UniversalElementRole;

    /// Returns the role of this element type.
    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

/// Converts a DartTokenType to a DartElementType.
impl From<crate::lexer::token_type::DartTokenType> for DartElementType {
    /// Converts the given token type to its corresponding element type.
    fn from(token: crate::lexer::token_type::DartTokenType) -> Self {
        match token {
            crate::lexer::token_type::DartTokenType::Root => DartElementType::Root,
            crate::lexer::token_type::DartTokenType::ClassDeclaration => DartElementType::ClassDeclaration,
            crate::lexer::token_type::DartTokenType::FunctionDeclaration => DartElementType::FunctionDeclaration,
            crate::lexer::token_type::DartTokenType::Whitespace => DartElementType::Whitespace,
            crate::lexer::token_type::DartTokenType::Newline => DartElementType::Newline,
            crate::lexer::token_type::DartTokenType::Identifier => DartElementType::Identifier,
            crate::lexer::token_type::DartTokenType::IntegerLiteral => DartElementType::IntegerLiteral,
            crate::lexer::token_type::DartTokenType::DoubleLiteral => DartElementType::DoubleLiteral,
            crate::lexer::token_type::DartTokenType::StringLiteral => DartElementType::StringLiteral,
            crate::lexer::token_type::DartTokenType::BooleanLiteral => DartElementType::BooleanLiteral,
            crate::lexer::token_type::DartTokenType::NullLiteral => DartElementType::NullLiteral,
            crate::lexer::token_type::DartTokenType::Abstract => DartElementType::Abstract,
            crate::lexer::token_type::DartTokenType::As => DartElementType::As,
            crate::lexer::token_type::DartTokenType::Assert => DartElementType::Assert,
            crate::lexer::token_type::DartTokenType::Async => DartElementType::Async,
            crate::lexer::token_type::DartTokenType::Await => DartElementType::Await,
            crate::lexer::token_type::DartTokenType::Break => DartElementType::Break,
            crate::lexer::token_type::DartTokenType::Case => DartElementType::Case,
            crate::lexer::token_type::DartTokenType::Catch => DartElementType::Catch,
            crate::lexer::token_type::DartTokenType::Class => DartElementType::Class,
            crate::lexer::token_type::DartTokenType::Const => DartElementType::Const,
            crate::lexer::token_type::DartTokenType::Continue => DartElementType::Continue,
            crate::lexer::token_type::DartTokenType::Covariant => DartElementType::Covariant,
            crate::lexer::token_type::DartTokenType::Default => DartElementType::Default,
            crate::lexer::token_type::DartTokenType::Deferred => DartElementType::Deferred,
            crate::lexer::token_type::DartTokenType::Do => DartElementType::Do,
            crate::lexer::token_type::DartTokenType::Dynamic => DartElementType::Dynamic,
            crate::lexer::token_type::DartTokenType::Else => DartElementType::Else,
            crate::lexer::token_type::DartTokenType::Enum => DartElementType::Enum,
            crate::lexer::token_type::DartTokenType::Export => DartElementType::Export,
            crate::lexer::token_type::DartTokenType::Extends => DartElementType::Extends,
            crate::lexer::token_type::DartTokenType::Extension => DartElementType::Extension,
            crate::lexer::token_type::DartTokenType::External => DartElementType::External,
            crate::lexer::token_type::DartTokenType::Factory => DartElementType::Factory,
            crate::lexer::token_type::DartTokenType::False => DartElementType::False,
            crate::lexer::token_type::DartTokenType::Final => DartElementType::Final,
            crate::lexer::token_type::DartTokenType::Finally => DartElementType::Finally,
            crate::lexer::token_type::DartTokenType::For => DartElementType::For,
            crate::lexer::token_type::DartTokenType::Function => DartElementType::Function,
            crate::lexer::token_type::DartTokenType::Get => DartElementType::Get,
            crate::lexer::token_type::DartTokenType::Hide => DartElementType::Hide,
            crate::lexer::token_type::DartTokenType::If => DartElementType::If,
            crate::lexer::token_type::DartTokenType::Implements => DartElementType::Implements,
            crate::lexer::token_type::DartTokenType::Import => DartElementType::Import,
            crate::lexer::token_type::DartTokenType::In => DartElementType::In,
            crate::lexer::token_type::DartTokenType::Interface => DartElementType::Interface,
            crate::lexer::token_type::DartTokenType::Int => DartElementType::Int,
            crate::lexer::token_type::DartTokenType::Is => DartElementType::Is,
            crate::lexer::token_type::DartTokenType::Late => DartElementType::Late,
            crate::lexer::token_type::DartTokenType::Library => DartElementType::Library,
            crate::lexer::token_type::DartTokenType::Mixin => DartElementType::Mixin,
            crate::lexer::token_type::DartTokenType::New => DartElementType::New,
            crate::lexer::token_type::DartTokenType::Null => DartElementType::Null,
            crate::lexer::token_type::DartTokenType::On => DartElementType::On,
            crate::lexer::token_type::DartTokenType::Operator => DartElementType::Operator,
            crate::lexer::token_type::DartTokenType::Part => DartElementType::Part,
            crate::lexer::token_type::DartTokenType::Required => DartElementType::Required,
            crate::lexer::token_type::DartTokenType::Rethrow => DartElementType::Rethrow,
            crate::lexer::token_type::DartTokenType::Return => DartElementType::Return,
            crate::lexer::token_type::DartTokenType::Set => DartElementType::Set,
            crate::lexer::token_type::DartTokenType::Show => DartElementType::Show,
            crate::lexer::token_type::DartTokenType::Static => DartElementType::Static,
            crate::lexer::token_type::DartTokenType::Super => DartElementType::Super,
            crate::lexer::token_type::DartTokenType::Switch => DartElementType::Switch,
            crate::lexer::token_type::DartTokenType::Sync => DartElementType::Sync,
            crate::lexer::token_type::DartTokenType::This => DartElementType::This,
            crate::lexer::token_type::DartTokenType::Throw => DartElementType::Throw,
            crate::lexer::token_type::DartTokenType::True => DartElementType::True,
            crate::lexer::token_type::DartTokenType::Try => DartElementType::Try,
            crate::lexer::token_type::DartTokenType::Typedef => DartElementType::Typedef,
            crate::lexer::token_type::DartTokenType::Var => DartElementType::Var,
            crate::lexer::token_type::DartTokenType::Void => DartElementType::Void,
            crate::lexer::token_type::DartTokenType::While => DartElementType::While,
            crate::lexer::token_type::DartTokenType::With => DartElementType::With,
            crate::lexer::token_type::DartTokenType::Yield => DartElementType::Yield,
            crate::lexer::token_type::DartTokenType::Plus => DartElementType::Plus,
            crate::lexer::token_type::DartTokenType::Minus => DartElementType::Minus,
            crate::lexer::token_type::DartTokenType::Star => DartElementType::Star,
            crate::lexer::token_type::DartTokenType::Slash => DartElementType::Slash,
            crate::lexer::token_type::DartTokenType::Percent => DartElementType::Percent,
            crate::lexer::token_type::DartTokenType::TildeSlash => DartElementType::TildeSlash,
            crate::lexer::token_type::DartTokenType::Equal => DartElementType::Equal,
            crate::lexer::token_type::DartTokenType::EqualEqual => DartElementType::EqualEqual,
            crate::lexer::token_type::DartTokenType::BangEqual => DartElementType::BangEqual,
            crate::lexer::token_type::DartTokenType::Less => DartElementType::Less,
            crate::lexer::token_type::DartTokenType::Greater => DartElementType::Greater,
            crate::lexer::token_type::DartTokenType::LessEqual => DartElementType::LessEqual,
            crate::lexer::token_type::DartTokenType::GreaterEqual => DartElementType::GreaterEqual,
            crate::lexer::token_type::DartTokenType::LeftShift => DartElementType::LeftShift,
            crate::lexer::token_type::DartTokenType::RightShift => DartElementType::RightShift,
            crate::lexer::token_type::DartTokenType::Ampersand => DartElementType::Ampersand,
            crate::lexer::token_type::DartTokenType::Pipe => DartElementType::Pipe,
            crate::lexer::token_type::DartTokenType::Caret => DartElementType::Caret,
            crate::lexer::token_type::DartTokenType::Tilde => DartElementType::Tilde,
            crate::lexer::token_type::DartTokenType::Bang => DartElementType::Bang,
            crate::lexer::token_type::DartTokenType::AmpersandAmpersand => DartElementType::AmpersandAmpersand,
            crate::lexer::token_type::DartTokenType::PipePipe => DartElementType::PipePipe,
            crate::lexer::token_type::DartTokenType::Question => DartElementType::Question,
            crate::lexer::token_type::DartTokenType::QuestionQuestion => DartElementType::QuestionQuestion,
            crate::lexer::token_type::DartTokenType::PlusPlus => DartElementType::PlusPlus,
            crate::lexer::token_type::DartTokenType::MinusMinus => DartElementType::MinusMinus,
            crate::lexer::token_type::DartTokenType::PlusEqual => DartElementType::PlusEqual,
            crate::lexer::token_type::DartTokenType::MinusEqual => DartElementType::MinusEqual,
            crate::lexer::token_type::DartTokenType::StarEqual => DartElementType::StarEqual,
            crate::lexer::token_type::DartTokenType::SlashEqual => DartElementType::SlashEqual,
            crate::lexer::token_type::DartTokenType::PercentEqual => DartElementType::PercentEqual,
            crate::lexer::token_type::DartTokenType::TildeSlashEqual => DartElementType::TildeSlashEqual,
            crate::lexer::token_type::DartTokenType::LeftShiftEqual => DartElementType::LeftShiftEqual,
            crate::lexer::token_type::DartTokenType::RightShiftEqual => DartElementType::RightShiftEqual,
            crate::lexer::token_type::DartTokenType::AmpersandEqual => DartElementType::AmpersandEqual,
            crate::lexer::token_type::DartTokenType::PipeEqual => DartElementType::PipeEqual,
            crate::lexer::token_type::DartTokenType::CaretEqual => DartElementType::CaretEqual,
            crate::lexer::token_type::DartTokenType::QuestionQuestionEqual => DartElementType::QuestionQuestionEqual,
            crate::lexer::token_type::DartTokenType::Arrow => DartElementType::Arrow,
            crate::lexer::token_type::DartTokenType::Dot => DartElementType::Dot,
            crate::lexer::token_type::DartTokenType::DotDot => DartElementType::DotDot,
            crate::lexer::token_type::DartTokenType::DotDotDot => DartElementType::DotDotDot,
            crate::lexer::token_type::DartTokenType::QuestionDot => DartElementType::QuestionDot,
            crate::lexer::token_type::DartTokenType::LeftParen => DartElementType::LeftParen,
            crate::lexer::token_type::DartTokenType::RightParen => DartElementType::RightParen,
            crate::lexer::token_type::DartTokenType::LeftBracket => DartElementType::LeftBracket,
            crate::lexer::token_type::DartTokenType::RightBracket => DartElementType::RightBracket,
            crate::lexer::token_type::DartTokenType::LeftBrace => DartElementType::LeftBrace,
            crate::lexer::token_type::DartTokenType::RightBrace => DartElementType::RightBrace,
            crate::lexer::token_type::DartTokenType::Semicolon => DartElementType::Semicolon,
            crate::lexer::token_type::DartTokenType::Comma => DartElementType::Comma,
            crate::lexer::token_type::DartTokenType::Colon => DartElementType::Colon,
            crate::lexer::token_type::DartTokenType::At => DartElementType::At,
            crate::lexer::token_type::DartTokenType::Hash => DartElementType::Hash,
            crate::lexer::token_type::DartTokenType::LineComment => DartElementType::LineComment,
            crate::lexer::token_type::DartTokenType::BlockComment => DartElementType::BlockComment,
            crate::lexer::token_type::DartTokenType::DocComment => DartElementType::DocComment,
            crate::lexer::token_type::DartTokenType::Error => DartElementType::Error,
            crate::lexer::token_type::DartTokenType::Eof => DartElementType::Eof,
            crate::lexer::token_type::DartTokenType::VariableDeclaration => DartElementType::VariableDeclaration,
        }
    }
}
