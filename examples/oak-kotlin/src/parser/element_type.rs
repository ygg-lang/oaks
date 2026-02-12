use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Kotlin parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KotlinElementType {
    // Structural nodes
    /// A source file node.
    SourceFile,
    /// A class declaration.
    ClassDeclaration,
    /// A function declaration.
    FunctionDeclaration,
    /// A variable declaration.
    VariableDeclaration,
    /// An `if` statement.
    IfStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `return` statement.
    ReturnStatement,
    /// A block of statements.
    Block,
    /// A parameter in a function or constructor.
    Parameter,
    /// An identifier.
    Identifier,
    /// An integer literal.
    IntLiteral,
    /// A prefix expression (e.g., `-x`, `++x`).
    PrefixExpression,
    /// A member access expression (e.g., `obj.prop`).
    MemberAccessExpression,
    /// A function call expression.
    CallExpression,
    /// An assignment expression.
    AssignmentExpression,
    /// A binary expression (e.g., `x + y`).
    BinaryExpression,
    /// An error node in the parse tree.
    Error,

    // Mirrored token kinds (for leaf elements and completeness)
    /// The root of the parse tree.
    Root,
    /// End of stream.
    EndOfStream,
    /// A unary expression.
    UnaryExpression,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A type reference.
    TypeReference,
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
    /// The `public` visibility modifier.
    Public,
    /// The `private` visibility modifier.
    Private,
    /// The `protected` visibility modifier.
    Protected,
    /// The `internal` visibility modifier.
    Internal,
    /// The `abstract` modifier.
    Abstract,
    /// The `final` modifier.
    Final,
    /// The `open` modifier.
    Open,
    /// The `override` modifier.
    Override,
    /// The `companion` keyword.
    Companion,
    /// The `object` keyword.
    Object,
    /// The `interface` keyword.
    Interface,
    /// The `enum` keyword.
    Enum,
    /// The `data` modifier.
    Data,
    /// The `sealed` modifier.
    Sealed,
    /// The `inline` modifier.
    Inline,
    /// The `suspend` modifier.
    Suspend,
    /// The `operator` modifier.
    Operator,
    /// The `infix` modifier.
    Infix,
    /// The `tailrec` modifier.
    Tailrec,
    /// The `external` modifier.
    External,
    /// The `annotation` keyword.
    Annotation,
    /// The `crossinline` modifier.
    Crossinline,
    /// The `noinline` modifier.
    Noinline,
    /// The `reified` modifier.
    Reified,
    /// The `vararg` modifier.
    Vararg,
    /// The `out` variance modifier.
    Out,
    /// The `in` variance modifier.
    In,
    /// The `is` keyword.
    Is,
    /// The `as` keyword.
    As,
    /// The `this` keyword.
    This,
    /// The `super` keyword.
    Super,
    /// The `null` literal.
    Null,
    /// The `true` literal.
    True,
    /// The `false` literal.
    False,
    /// A keyword (generic).
    Keyword,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A numeric literal (generic).
    NumberLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A boolean literal.
    BooleanLiteral,
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
    Equals,
    /// The `<` operator.
    Less,
    /// The `>` operator.
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
    /// Opening parenthesis (`(`).
    LParen,
    /// Closing parenthesis (`)`).
    RParen,
    /// Opening bracket (`[`).
    LBracket,
    /// Closing bracket (`]`).
    RBracket,
    /// Opening brace (`{`).
    LBrace,
    /// Closing brace (`}`).
    RBrace,
    /// A comment.
    Comment,
    /// Whitespace characters.
    Whitespace,
    /// A newline character.
    Newline,
}

impl ElementType for KotlinElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            KotlinElementType::SourceFile | KotlinElementType::Root => UniversalElementRole::Root,
            KotlinElementType::ClassDeclaration | KotlinElementType::FunctionDeclaration => UniversalElementRole::Definition,
            KotlinElementType::VariableDeclaration | KotlinElementType::IfStatement | KotlinElementType::WhileStatement | KotlinElementType::ReturnStatement | KotlinElementType::Block => UniversalElementRole::Statement,
            KotlinElementType::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::KotlinTokenType> for KotlinElementType {
    fn from(token: crate::lexer::token_type::KotlinTokenType) -> Self {
        use crate::lexer::token_type::KotlinTokenType as T;
        match token {
            T::Root => KotlinElementType::Root,
            T::SourceFile => KotlinElementType::SourceFile,
            T::EndOfStream => KotlinElementType::EndOfStream,
            T::FunctionDeclaration => KotlinElementType::FunctionDeclaration,
            T::ClassDeclaration => KotlinElementType::ClassDeclaration,
            T::VariableDeclaration => KotlinElementType::VariableDeclaration,
            T::IfStatement => KotlinElementType::IfStatement,
            T::WhileStatement => KotlinElementType::WhileStatement,
            T::ReturnStatement => KotlinElementType::ReturnStatement,
            T::Block => KotlinElementType::Block,
            T::BinaryExpression => KotlinElementType::BinaryExpression,
            T::UnaryExpression => KotlinElementType::UnaryExpression,
            T::AssignmentExpression => KotlinElementType::AssignmentExpression,
            T::CallExpression => KotlinElementType::CallExpression,
            T::MemberAccessExpression => KotlinElementType::MemberAccessExpression,
            T::LiteralExpression => KotlinElementType::LiteralExpression,
            T::IdentifierExpression => KotlinElementType::IdentifierExpression,
            T::Parameter => KotlinElementType::Parameter,
            T::TypeReference => KotlinElementType::TypeReference,
            T::Class => KotlinElementType::Class,
            T::Fun => KotlinElementType::Fun,
            T::Val => KotlinElementType::Val,
            T::Var => KotlinElementType::Var,
            T::If => KotlinElementType::If,
            T::Else => KotlinElementType::Else,
            T::When => KotlinElementType::When,
            T::For => KotlinElementType::For,
            T::While => KotlinElementType::While,
            T::Return => KotlinElementType::Return,
            T::Break => KotlinElementType::Break,
            T::Continue => KotlinElementType::Continue,
            T::Try => KotlinElementType::Try,
            T::Catch => KotlinElementType::Catch,
            T::Finally => KotlinElementType::Finally,
            T::Throw => KotlinElementType::Throw,
            T::Import => KotlinElementType::Import,
            T::Package => KotlinElementType::Package,
            T::Public => KotlinElementType::Public,
            T::Private => KotlinElementType::Private,
            T::Protected => KotlinElementType::Protected,
            T::Internal => KotlinElementType::Internal,
            T::Abstract => KotlinElementType::Abstract,
            T::Final => KotlinElementType::Final,
            T::Open => KotlinElementType::Open,
            T::Override => KotlinElementType::Override,
            T::Companion => KotlinElementType::Companion,
            T::Object => KotlinElementType::Object,
            T::Interface => KotlinElementType::Interface,
            T::Enum => KotlinElementType::Enum,
            T::Data => KotlinElementType::Data,
            T::Sealed => KotlinElementType::Sealed,
            T::Inline => KotlinElementType::Inline,
            T::Suspend => KotlinElementType::Suspend,
            T::Operator => KotlinElementType::Operator,
            T::Infix => KotlinElementType::Infix,
            T::Tailrec => KotlinElementType::Tailrec,
            T::External => KotlinElementType::External,
            T::Annotation => KotlinElementType::Annotation,
            T::Crossinline => KotlinElementType::Crossinline,
            T::Noinline => KotlinElementType::Noinline,
            T::Reified => KotlinElementType::Reified,
            T::Vararg => KotlinElementType::Vararg,
            T::Out => KotlinElementType::Out,
            T::In => KotlinElementType::In,
            T::Is => KotlinElementType::Is,
            T::As => KotlinElementType::As,
            T::This => KotlinElementType::This,
            T::Super => KotlinElementType::Super,
            T::Null => KotlinElementType::Null,
            T::True => KotlinElementType::True,
            T::False => KotlinElementType::False,
            T::Identifier => KotlinElementType::Identifier,
            T::Keyword => KotlinElementType::Keyword,
            T::StringLiteral => KotlinElementType::StringLiteral,
            T::CharLiteral => KotlinElementType::CharLiteral,
            T::NumberLiteral => KotlinElementType::NumberLiteral,
            T::IntLiteral => KotlinElementType::IntLiteral,
            T::FloatLiteral => KotlinElementType::FloatLiteral,
            T::BooleanLiteral => KotlinElementType::BooleanLiteral,
            T::Plus => KotlinElementType::Plus,
            T::Minus => KotlinElementType::Minus,
            T::Star => KotlinElementType::Star,
            T::Slash => KotlinElementType::Slash,
            T::Percent => KotlinElementType::Percent,
            T::Equals => KotlinElementType::Equals,
            T::Less => KotlinElementType::Less,
            T::Greater => KotlinElementType::Greater,
            T::Ampersand => KotlinElementType::Ampersand,
            T::Pipe => KotlinElementType::Pipe,
            T::Caret => KotlinElementType::Caret,
            T::Tilde => KotlinElementType::Tilde,
            T::Exclamation => KotlinElementType::Exclamation,
            T::Assign => KotlinElementType::Assign,
            T::PlusAssign => KotlinElementType::PlusAssign,
            T::MinusAssign => KotlinElementType::MinusAssign,
            T::StarAssign => KotlinElementType::StarAssign,
            T::SlashAssign => KotlinElementType::SlashAssign,
            T::PercentAssign => KotlinElementType::PercentAssign,
            T::EqEq => KotlinElementType::EqEq,
            T::NotEq => KotlinElementType::NotEq,
            T::Lt => KotlinElementType::Lt,
            T::Gt => KotlinElementType::Gt,
            T::LtEq => KotlinElementType::LtEq,
            T::GtEq => KotlinElementType::GtEq,
            T::AndAnd => KotlinElementType::AndAnd,
            T::OrOr => KotlinElementType::OrOr,
            T::Dot => KotlinElementType::Dot,
            T::Comma => KotlinElementType::Comma,
            T::Colon => KotlinElementType::Colon,
            T::Semi => KotlinElementType::Semi,
            T::Arrow => KotlinElementType::Arrow,
            T::DoubleColon => KotlinElementType::DoubleColon,
            T::Range => KotlinElementType::Range,
            T::Question => KotlinElementType::Question,
            T::ExclamationExclamation => KotlinElementType::ExclamationExclamation,
            T::At => KotlinElementType::At,
            T::LParen => KotlinElementType::LParen,
            T::RParen => KotlinElementType::RParen,
            T::LBracket => KotlinElementType::LBracket,
            T::RBracket => KotlinElementType::RBracket,
            T::LBrace => KotlinElementType::LBrace,
            T::RBrace => KotlinElementType::RBrace,
            T::Comment => KotlinElementType::Comment,
            T::Whitespace => KotlinElementType::Whitespace,
            T::Newline => KotlinElementType::Newline,
            T::Error => KotlinElementType::Error,
        }
    }
}
