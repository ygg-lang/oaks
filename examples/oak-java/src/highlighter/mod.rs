use crate::kind::JavaSyntaxKind;

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 字面量
    Literal,
    /// 标识符
    Identifier,
    /// 注释
    Comment,
    /// 运算符
    Operator,
    /// 标点符号
    Punctuation,
    /// 函数/方法
    Function,
    /// 类型/类
    Type,
}

pub struct JavaHighlighter;

impl JavaHighlighter {
    pub fn highlight(&self, kind: JavaSyntaxKind) -> Option<HighlightKind> {
        match kind {
            JavaSyntaxKind::Abstract
            | JavaSyntaxKind::Assert
            | JavaSyntaxKind::Boolean
            | JavaSyntaxKind::Break
            | JavaSyntaxKind::Byte
            | JavaSyntaxKind::Case
            | JavaSyntaxKind::Catch
            | JavaSyntaxKind::Char
            | JavaSyntaxKind::Class
            | JavaSyntaxKind::Const
            | JavaSyntaxKind::Continue
            | JavaSyntaxKind::Default
            | JavaSyntaxKind::Do
            | JavaSyntaxKind::Double
            | JavaSyntaxKind::Else
            | JavaSyntaxKind::Enum
            | JavaSyntaxKind::Extends
            | JavaSyntaxKind::Final
            | JavaSyntaxKind::Finally
            | JavaSyntaxKind::Float
            | JavaSyntaxKind::For
            | JavaSyntaxKind::If
            | JavaSyntaxKind::Goto
            | JavaSyntaxKind::Implements
            | JavaSyntaxKind::Import
            | JavaSyntaxKind::Instanceof
            | JavaSyntaxKind::Int
            | JavaSyntaxKind::Interface
            | JavaSyntaxKind::Long
            | JavaSyntaxKind::Native
            | JavaSyntaxKind::New
            | JavaSyntaxKind::Package
            | JavaSyntaxKind::Private
            | JavaSyntaxKind::Protected
            | JavaSyntaxKind::Public
            | JavaSyntaxKind::Return
            | JavaSyntaxKind::Short
            | JavaSyntaxKind::Static
            | JavaSyntaxKind::Strictfp
            | JavaSyntaxKind::Super
            | JavaSyntaxKind::Switch
            | JavaSyntaxKind::Synchronized
            | JavaSyntaxKind::This
            | JavaSyntaxKind::Throw
            | JavaSyntaxKind::Throws
            | JavaSyntaxKind::Transient
            | JavaSyntaxKind::Try
            | JavaSyntaxKind::Void
            | JavaSyntaxKind::Volatile
            | JavaSyntaxKind::While => Some(HighlightKind::Keyword),

            JavaSyntaxKind::IntegerLiteral | JavaSyntaxKind::FloatingPointLiteral | JavaSyntaxKind::BooleanLiteral | JavaSyntaxKind::CharacterLiteral | JavaSyntaxKind::StringLiteral | JavaSyntaxKind::NullLiteral => Some(HighlightKind::Literal),

            JavaSyntaxKind::Identifier => Some(HighlightKind::Identifier),

            JavaSyntaxKind::LineComment | JavaSyntaxKind::BlockComment => Some(HighlightKind::Comment),

            JavaSyntaxKind::Assign
            | JavaSyntaxKind::GreaterThan
            | JavaSyntaxKind::LessThan
            | JavaSyntaxKind::Bang
            | JavaSyntaxKind::Tilde
            | JavaSyntaxKind::Question
            | JavaSyntaxKind::Colon
            | JavaSyntaxKind::Equals
            | JavaSyntaxKind::LessThanEquals
            | JavaSyntaxKind::GreaterThanEquals
            | JavaSyntaxKind::BangEquals
            | JavaSyntaxKind::AmpersandAmpersand
            | JavaSyntaxKind::PipePipe
            | JavaSyntaxKind::PlusPlus
            | JavaSyntaxKind::MinusMinus
            | JavaSyntaxKind::Plus
            | JavaSyntaxKind::Minus
            | JavaSyntaxKind::Asterisk
            | JavaSyntaxKind::Slash
            | JavaSyntaxKind::Ampersand
            | JavaSyntaxKind::Pipe
            | JavaSyntaxKind::Caret
            | JavaSyntaxKind::Percent
            | JavaSyntaxKind::LeftShift
            | JavaSyntaxKind::RightShift
            | JavaSyntaxKind::UnsignedRightShift
            | JavaSyntaxKind::PlusEquals
            | JavaSyntaxKind::MinusEquals
            | JavaSyntaxKind::AsteriskEquals
            | JavaSyntaxKind::SlashEquals
            | JavaSyntaxKind::AmpersandEquals
            | JavaSyntaxKind::PipeEquals
            | JavaSyntaxKind::CaretEquals
            | JavaSyntaxKind::PercentEquals
            | JavaSyntaxKind::LeftShiftEquals
            | JavaSyntaxKind::RightShiftEquals
            | JavaSyntaxKind::UnsignedRightShiftEquals => Some(HighlightKind::Operator),

            JavaSyntaxKind::LeftParen
            | JavaSyntaxKind::RightParen
            | JavaSyntaxKind::LeftBrace
            | JavaSyntaxKind::RightBrace
            | JavaSyntaxKind::LeftBracket
            | JavaSyntaxKind::RightBracket
            | JavaSyntaxKind::Semicolon
            | JavaSyntaxKind::Comma
            | JavaSyntaxKind::Dot
            | JavaSyntaxKind::Ellipsis
            | JavaSyntaxKind::At
            | JavaSyntaxKind::DoubleColon => Some(HighlightKind::Punctuation),

            _ => None,
        }
    }
}
