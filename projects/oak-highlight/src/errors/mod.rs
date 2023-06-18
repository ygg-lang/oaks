use oak_core::errors::OakError;
use std::string::String;

/// 扩展 ParseError 以支持高亮相关的错误
/// 通过为 ParseError 实现扩展方法来添加高亮特定的错误类型
pub trait ParseErrorExtHighlightExt {
    /// 创建无效主题错误
    fn invalid_theme(message: impl Into<String>) -> OakError;

    /// 创建不支持格式错误
    fn unsupported_format(format: impl Into<String>) -> OakError;

    /// 创建颜色解析错误
    fn color_parse_error(color: impl Into<String>) -> OakError;
}

impl ParseErrorExtHighlightExt for OakError {
    fn invalid_theme(_message: impl Into<String>) -> OakError {
        todo!()
    }

    fn unsupported_format(_format: impl Into<String>) -> OakError {
        todo!()
    }

    fn color_parse_error(_color: impl Into<String>) -> OakError {
        todo!()
    }
}

/// 高亮结果类型 - 现在直接使用 ParseError
pub type HighlightResult<T> = Result<T, OakError>;
