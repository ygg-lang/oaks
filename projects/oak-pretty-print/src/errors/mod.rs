// TODO: 这些类型在 oak-core 中不存在，需要实现
// pub use oak_core::{Diagnostic, DiagnosticLevel, ParseError, PexError, PexErrorKind};

use core::fmt;

/// 格式化特定的错误类型，扩展 oak-core 的错误系统
#[derive(Debug, Clone, PartialEq)]
pub enum FormatError {
    /// 无效的配置
    InvalidConfig { message: String },
    /// 格式化规则冲突
    RuleConflict { rule1: String, rule2: String },
    /// 不支持的语言特性
    UnsupportedFeature { feature: String },
    /// IO 错误
    IoError { message: String },
    // TODO: 包装 oak-core 的解析错误
    // CoreParseError(ParseError),
    // TODO: 包装 oak-core 的通用错误
    // CoreError(PexError),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::InvalidConfig { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
            FormatError::RuleConflict { rule1, rule2 } => {
                write!(f, "Rule conflict between '{}' and '{}'", rule1, rule2)
            }
            FormatError::UnsupportedFeature { feature } => {
                write!(f, "Unsupported feature: {}", feature)
            }
            FormatError::IoError { message } => {
                write!(f, "IO error: {}", message)
            } /* TODO: 实现对 oak-core 错误的处理
               * FormatError::CoreParseError(err) => {
               *     write!(f, "Parse error: {:?}", err)
               * }
               * FormatError::CoreError(err) => {
               *     write!(f, "Core error: {}", err.message)
               * } */
        }
    }
}

// TODO: 实现对 oak-core 错误类型的转换
// impl From<ParseError> for FormatError {
//     fn from(err: ParseError) -> Self {
//         FormatError::CoreParseError(err)
//     }
// }

// impl From<PexError> for FormatError {
//     fn from(err: PexError) -> Self {
//         FormatError::CoreError(err)
//     }
// }

/// 格式化结果类型
pub type FormatResult<T> = Result<T, FormatError>;
