#![doc = include_str!("readme.md")]
use crate::JavaTokenType;

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
    pub fn highlight(&self, kind: JavaTokenType) -> Option<HighlightKind> {
        match kind {
            JavaTokenType::Abstract
            | JavaTokenType::Assert
            | JavaTokenType::Boolean
            | JavaTokenType::Break
            | JavaTokenType::Byte
            | JavaTokenType::Case
            | JavaTokenType::Catch
            | JavaTokenType::Char
            | JavaTokenType::Class
            | JavaTokenType::Const
            | JavaTokenType::Continue
            | JavaTokenType::Default
            | JavaTokenType::Do
            | JavaTokenType::Double
            | JavaTokenType::Else
            | JavaTokenType::Enum
            | JavaTokenType::Extends
            | JavaTokenType::Final
            | JavaTokenType::Finally
            | JavaTokenType::Float
            | JavaTokenType::For
            | JavaTokenType::If
            | JavaTokenType::Goto
            | JavaTokenType::Implements
            | JavaTokenType::Import
            | JavaTokenType::Instanceof
            | JavaTokenType::Int
            | JavaTokenType::Interface
            | JavaTokenType::Long
            | JavaTokenType::Native
            | JavaTokenType::New
            | JavaTokenType::Package
            | JavaTokenType::Private
            | JavaTokenType::Protected
            | JavaTokenType::Public
            | JavaTokenType::Return
            | JavaTokenType::Short
            | JavaTokenType::Static
            | JavaTokenType::Strictfp
            | JavaTokenType::Super
            | JavaTokenType::Switch
            | JavaTokenType::Synchronized
            | JavaTokenType::This
            | JavaTokenType::Throw
            | JavaTokenType::Throws
            | JavaTokenType::Transient
            | JavaTokenType::Try
            | JavaTokenType::Void
            | JavaTokenType::Volatile
            | JavaTokenType::While => Some(HighlightKind::Keyword),

            JavaTokenType::IntegerLiteral | JavaTokenType::FloatingPointLiteral | JavaTokenType::BooleanLiteral | JavaTokenType::CharacterLiteral | JavaTokenType::StringLiteral | JavaTokenType::NullLiteral => Some(HighlightKind::Literal),

            JavaTokenType::Identifier => Some(HighlightKind::Identifier),

            JavaTokenType::LineComment | JavaTokenType::BlockComment => Some(HighlightKind::Comment),

            JavaTokenType::Assign
            | JavaTokenType::GreaterThan
            | JavaTokenType::LessThan
            | JavaTokenType::Bang
            | JavaTokenType::Tilde
            | JavaTokenType::Question
            | JavaTokenType::Colon
            | JavaTokenType::Equals
            | JavaTokenType::LessThanEquals
            | JavaTokenType::GreaterThanEquals
            | JavaTokenType::BangEquals
            | JavaTokenType::AmpersandAmpersand
            | JavaTokenType::PipePipe
            | JavaTokenType::PlusPlus
            | JavaTokenType::MinusMinus
            | JavaTokenType::Plus
            | JavaTokenType::Minus
            | JavaTokenType::Asterisk
            | JavaTokenType::Slash
            | JavaTokenType::Ampersand
            | JavaTokenType::Pipe
            | JavaTokenType::Caret
            | JavaTokenType::Percent
            | JavaTokenType::LeftShift
            | JavaTokenType::RightShift
            | JavaTokenType::UnsignedRightShift
            | JavaTokenType::PlusEquals
            | JavaTokenType::MinusEquals
            | JavaTokenType::AsteriskEquals
            | JavaTokenType::SlashEquals
            | JavaTokenType::AmpersandEquals
            | JavaTokenType::PipeEquals
            | JavaTokenType::CaretEquals
            | JavaTokenType::PercentEquals
            | JavaTokenType::LeftShiftEquals
            | JavaTokenType::RightShiftEquals
            | JavaTokenType::UnsignedRightShiftEquals => Some(HighlightKind::Operator),

            JavaTokenType::LeftParen
            | JavaTokenType::RightParen
            | JavaTokenType::LeftBrace
            | JavaTokenType::RightBrace
            | JavaTokenType::LeftBracket
            | JavaTokenType::RightBracket
            | JavaTokenType::Semicolon
            | JavaTokenType::Comma
            | JavaTokenType::Dot
            | JavaTokenType::Ellipsis
            | JavaTokenType::At
            | JavaTokenType::DoubleColon => Some(HighlightKind::Punctuation),

            _ => None,
        }
    }
}
