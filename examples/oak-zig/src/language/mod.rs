use crate::kind::ZigSyntaxKind;
use oak_core::Language;

pub struct ZigLanguage {}

impl Language for ZigLanguage {
    type SyntaxKind = ZigSyntaxKind;

    fn line_comment_start(&self) -> Option<&'static str> {
        Some("//")
    }

    fn block_comment_start(&self) -> Option<&'static str> {
        None // Zig 不支持块注释
    }

    fn block_comment_end(&self) -> Option<&'static str> {
        None
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['"']
    }

    fn string_escape_char(&self) -> Option<char> {
        Some('\\')
    }

    fn whitespace_chars(&self) -> &'static [char] {
        &[' ', '\t']
    }

    fn newline_chars(&self) -> &'static [char] {
        &['\n', '\r']
    }
}
