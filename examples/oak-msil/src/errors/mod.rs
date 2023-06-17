use alloc::{fmt, string::String};
use core::ops::Range;

/// MSIL 解析错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum MsilError {
    /// 词法分析错误
    LexError {
        /// 错误消息
        message: String,
        /// 错误位置
        span: Range<usize>,
    },
    /// 语法分析错误
    ParseError {
        /// 错误消息
        message: String,
        /// 错误位置
        span: Range<usize>,
    },
    /// 语义分析错误
    SemanticError {
        /// 错误消息
        message: String,
        /// 错误位置
        span: Range<usize>,
    },
    /// 未预期的 Token
    UnexpectedToken {
        /// 实际Token
        actual: String,
        /// 期望Token
        expected: String,
        /// 错误位置
        span: Range<usize>,
    },
    /// 未预期的文件结束
    UnexpectedEof {
        /// 期望的内
        expected: String,
        /// 错误位置
        span: Range<usize>,
    },
}

impl MsilError {
    /// 获取错误位置
    pub fn span(&self) -> &Range<usize> {
        match self {
            MsilError::LexError { span, .. }
            | MsilError::ParseError { span, .. }
            | MsilError::SemanticError { span, .. }
            | MsilError::UnexpectedToken { span, .. }
            | MsilError::UnexpectedEof { span, .. } => span,
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> &str {
        match self {
            MsilError::LexError { message, .. }
            | MsilError::ParseError { message, .. }
            | MsilError::SemanticError { message, .. } => message,
            MsilError::UnexpectedToken { actual, .. } => {
                // 这里简化处理，实际应该返回格式化的消息
                actual
            }
            MsilError::UnexpectedEof { expected, .. } => expected,
        }
    }
}

impl fmt::Display for MsilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MsilError::LexError { message, span } => {
                write!(f, "Lexical error at {}..{}: {}", span.start, span.end, message)
            }
            MsilError::ParseError { message, span } => {
                write!(f, "Parse error at {}..{}: {}", span.start, span.end, message)
            }
            MsilError::SemanticError { message, span } => {
                write!(f, "Semantic error at {}..{}: {}", span.start, span.end, message)
            }
            MsilError::UnexpectedToken { actual, expected, span } => {
                write!(f, "Unexpected kind at {}..{}: expected '{}', found '{}'", span.start, span.end, expected, actual)
            }
            MsilError::UnexpectedEof { expected, span } => {
                write!(f, "Unexpected end of file at {}..{}: expected '{}'", span.start, span.end, expected)
            }
        }
    }
}

/// MSIL 解析结果类型
pub type MsilResult<T> = Result<T, MsilError>;
