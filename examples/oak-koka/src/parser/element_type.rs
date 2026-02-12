use oak_core::{ElementType, UniversalElementRole};
/// Element types for the Koka parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KokaElementType {
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

impl ElementType for KokaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            KokaElementType::SourceFile | KokaElementType::Root => UniversalElementRole::Root,
            KokaElementType::ClassDeclaration | KokaElementType::FunctionDeclaration => UniversalElementRole::Definition,
            KokaElementType::VariableDeclaration | KokaElementType::IfStatement | KokaElementType::WhileStatement | KokaElementType::ReturnStatement | KokaElementType::Block => UniversalElementRole::Statement,
            KokaElementType::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::KokaTokenType> for KokaElementType {
    fn from(token: crate::lexer::token_type::KokaTokenType) -> Self {
        use crate::lexer::token_type::KokaTokenType as T;
        match token {
            T::Root => KokaElementType::Root,
            T::SourceFile => KokaElementType::SourceFile,
            T::EndOfStream => KokaElementType::EndOfStream,
            T::FunctionDeclaration => KokaElementType::FunctionDeclaration,
            T::ClassDeclaration => KokaElementType::ClassDeclaration,
            T::VariableDeclaration => KokaElementType::VariableDeclaration,
            T::IfStatement => KokaElementType::IfStatement,
            T::WhileStatement => KokaElementType::WhileStatement,
            T::ReturnStatement => KokaElementType::ReturnStatement,
            T::Block => KokaElementType::Block,
            T::BinaryExpression => KokaElementType::BinaryExpression,
            T::UnaryExpression => KokaElementType::UnaryExpression,
            T::AssignmentExpression => KokaElementType::AssignmentExpression,
            T::CallExpression => KokaElementType::CallExpression,
            T::MemberAccessExpression => KokaElementType::MemberAccessExpression,
            T::LiteralExpression => KokaElementType::LiteralExpression,
            T::IdentifierExpression => KokaElementType::IdentifierExpression,
            T::Parameter => KokaElementType::Parameter,
            T::TypeReference => KokaElementType::TypeReference,
            T::Class => KokaElementType::Class,
            T::Fun => KokaElementType::Fun,
            T::Val => KokaElementType::Val,
            T::Var => KokaElementType::Var,
            T::If => KokaElementType::If,
            T::Else => KokaElementType::Else,
            T::While => KokaElementType::While,
            T::Return => KokaElementType::Return,
            T::Import => KokaElementType::Import,
            T::Pub => KokaElementType::Public, // Map Pub token to Public element type
            T::Is => KokaElementType::Is,
            T::As => KokaElementType::As,
            T::Identifier => KokaElementType::Identifier,
            T::StringLiteral => KokaElementType::StringLiteral,
            T::CharLiteral => KokaElementType::CharLiteral,
            T::NumberLiteral => KokaElementType::NumberLiteral,
            T::BooleanLiteral => KokaElementType::BooleanLiteral,
            T::Plus => KokaElementType::Plus,
            T::Minus => KokaElementType::Minus,
            T::Star => KokaElementType::Star,
            T::Slash => KokaElementType::Slash,
            T::Percent => KokaElementType::Percent,
            T::Ampersand => KokaElementType::Ampersand,
            T::Pipe => KokaElementType::Pipe,
            T::Caret => KokaElementType::Caret,
            T::Tilde => KokaElementType::Tilde,
            T::Exclamation => KokaElementType::Exclamation,
            T::Assign => KokaElementType::Assign,
            T::PlusAssign => KokaElementType::PlusAssign,
            T::MinusAssign => KokaElementType::MinusAssign,
            T::StarAssign => KokaElementType::StarAssign,
            T::SlashAssign => KokaElementType::SlashAssign,
            T::PercentAssign => KokaElementType::PercentAssign,
            T::EqEq => KokaElementType::EqEq,
            T::NotEq => KokaElementType::NotEq,
            T::Lt => KokaElementType::Lt,
            T::Gt => KokaElementType::Gt,
            T::LtEq => KokaElementType::LtEq,
            T::GtEq => KokaElementType::GtEq,
            T::AndAnd => KokaElementType::AndAnd,
            T::OrOr => KokaElementType::OrOr,
            T::Dot => KokaElementType::Dot,
            T::Comma => KokaElementType::Comma,
            T::Colon => KokaElementType::Colon,
            T::Semi => KokaElementType::Semi,
            T::Arrow => KokaElementType::Arrow,
            T::DoubleColon => KokaElementType::DoubleColon,
            T::Range => KokaElementType::Range,
            T::Question => KokaElementType::Question,
            T::ExclamationExclamation => KokaElementType::ExclamationExclamation,
            T::At => KokaElementType::At,
            T::LParen => KokaElementType::LParen,
            T::RParen => KokaElementType::RParen,
            T::LBracket => KokaElementType::LBracket,
            T::RBracket => KokaElementType::RBracket,
            T::LBrace => KokaElementType::LBrace,
            T::RBrace => KokaElementType::RBrace,
            T::Comment => KokaElementType::Comment,
            T::Whitespace => KokaElementType::Whitespace,
            T::Newline => KokaElementType::Newline,
            T::Error => KokaElementType::Error,
            _ => KokaElementType::Error,
        }
    }
}
