use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for PureScript AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PurescriptElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// `ado`
    Ado,
    /// `case`
    Case,
    /// `class`
    Class,
    /// `data`
    Data,
    /// `derive`
    Derive,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `false`
    False,
    /// `forall`
    Forall,
    /// `foreign`
    Foreign,
    /// `if`
    If,
    /// `import`
    Import,
    /// `in`
    In,
    /// `infix`
    Infix,
    /// `infixl`
    Infixl,
    /// `infixr`
    Infixr,
    /// `instance`
    Instance,
    /// `let`
    Let,
    /// `module`
    Module,
    /// `newtype`
    Newtype,
    /// `of`
    Of,
    /// `then`
    Then,
    /// `true`
    True,
    /// `type`
    Type,
    /// `where`
    Where,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `\`
    Backslash,
    /// `|`
    Pipe,
    /// `=`
    Equal,
    /// `::`
    ColonColon,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `==`
    EqualEqual,
    /// `/=`
    NotEqual,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `!`
    Not,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `<>`
    Append,
    /// `<-`
    LeftArrow,
    /// `_`
    Underscore,
    /// Identifier.
    Identifier,
    /// Constructor.
    Constructor,
    /// Operator.
    Operator,
    /// String literal.
    StringLiteral,
    /// Integer literal.
    IntLiteral,
    /// Float literal.
    NumberLiteral,
    /// Char literal.
    CharLiteral,

    /// A module declaration.
    ModuleDeclaration,
    /// An import declaration.
    ImportDeclaration,
    /// A data declaration.
    DataDeclaration,
    /// A newtype declaration.
    NewtypeDeclaration,
    /// A type alias declaration.
    TypeAliasDeclaration,
    /// A class declaration.
    ClassDeclaration,
    /// An instance declaration.
    InstanceDeclaration,
    /// A foreign import declaration.
    ForeignImportDeclaration,
    /// A type signature.
    TypeSignature,
    /// A value declaration.
    ValueDeclaration,
    /// A pattern.
    Pattern,
    /// An expression.
    Expression,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A prefix expression.
    PrefixExpression,
    /// An infix expression.
    InfixExpression,
    /// A function application.
    ApplicationExpression,
    /// A lambda expression.
    LambdaExpression,
    /// A let expression.
    LetExpression,
    /// A case expression.
    CaseExpression,
    /// A case arm.
    CaseArm,
    /// A do expression.
    DoExpression,
    /// A type.
    TypeNode,

    /// Source file node.
    SourceFile,
    /// `<<<`
    Compose,
    /// `>>>`
    ComposeFlipped,
    /// `$`
    Apply,
    /// `#`
    ApplyFlipped,
    /// `>>=`
    Bind,
    /// `=<<`
    BindFlipped,
    /// `?`
    Question,
    /// `!`
    Exclamation,
    /// `@`
    At,
    /// Backtick
    Tick,
    /// Upper case identifier
    UpperIdentifier,
    /// Qualified identifier
    QualifiedIdentifier,
    /// Boolean literal
    BooleanLiteral,
    /// Root node
    Root,
    /// End of file
    Eof,
    /// Error node.
    Error,
}

impl ElementType for PurescriptElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PurescriptTokenType> for PurescriptElementType {
    fn from(token: crate::lexer::token_type::PurescriptTokenType) -> Self {
        use crate::lexer::token_type::PurescriptTokenType as T;
        match token {
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::Ado => Self::Ado,
            T::Case => Self::Case,
            T::Class => Self::Class,
            T::Data => Self::Data,
            T::Derive => Self::Derive,
            T::Do => Self::Do,
            T::Else => Self::Else,
            T::False => Self::False,
            T::Forall => Self::Forall,
            T::Foreign => Self::Foreign,
            T::If => Self::If,
            T::Import => Self::Import,
            T::In => Self::In,
            T::Infix => Self::Infix,
            T::Infixl => Self::Infixl,
            T::Infixr => Self::Infixr,
            T::Instance => Self::Instance,
            T::Let => Self::Let,
            T::Module => Self::Module,
            T::Newtype => Self::Newtype,
            T::Of => Self::Of,
            T::Then => Self::Then,
            T::True => Self::True,
            T::Type => Self::Type,
            T::Where => Self::Where,
            T::Arrow => Self::Arrow,
            T::FatArrow => Self::FatArrow,
            T::Backslash => Self::Backslash,
            T::Pipe => Self::Pipe,
            T::Equal => Self::Equal,
            T::ColonColon => Self::ColonColon,
            T::Dot => Self::Dot,
            T::DotDot => Self::DotDot,
            T::Plus => Self::Plus,
            T::Minus => Self::Minus,
            T::Star => Self::Star,
            T::Slash => Self::Slash,
            T::Percent => Self::Percent,
            T::Caret => Self::Caret,
            T::EqualEqual => Self::EqualEqual,
            T::NotEqual => Self::NotEqual,
            T::Less => Self::Less,
            T::Greater => Self::Greater,
            T::LessEqual => Self::LessEqual,
            T::GreaterEqual => Self::GreaterEqual,
            T::And => Self::And,
            T::Or => Self::Or,
            T::Append => Self::Append,
            T::Compose => Self::Compose,
            T::ComposeFlipped => Self::ComposeFlipped,
            T::Apply => Self::Apply,
            T::ApplyFlipped => Self::ApplyFlipped,
            T::Bind => Self::Bind,
            T::BindFlipped => Self::BindFlipped,
            T::LeftParen => Self::LeftParen,
            T::RightParen => Self::RightParen,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::Comma => Self::Comma,
            T::Semicolon => Self::Semicolon,
            T::Colon => Self::Colon,
            T::Question => Self::Question,
            T::Exclamation => Self::Exclamation,
            T::At => Self::At,
            T::Underscore => Self::Underscore,
            T::Tick => Self::Tick,
            T::IntLiteral => Self::IntLiteral,
            T::NumberLiteral => Self::NumberLiteral,
            T::StringLiteral => Self::StringLiteral,
            T::CharLiteral => Self::CharLiteral,
            T::BooleanLiteral => Self::BooleanLiteral,
            T::Identifier => Self::Identifier,
            T::UpperIdentifier => Self::UpperIdentifier,
            T::Operator => Self::Operator,
            T::QualifiedIdentifier => Self::QualifiedIdentifier,
            T::Root => Self::Root,
            T::SourceFile => Self::SourceFile,
            T::Error => Self::Error,
            T::Eof => Self::Eof,
        }
    }
}
