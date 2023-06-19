use oak_core::language::Language;
use crate::kind::LlvmKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LlvmLanguage;

impl Language for LlvmLanguage {
    type Kind = LlvmKind;

    fn kind_from_raw(raw: u16) -> Self::Kind {
        // Placeholder implementation
        match raw {
            0 => LlvmKind::Identifier,
            1 => LlvmKind::Number,
            2 => LlvmKind::String,
            3 => LlvmKind::Comment,
            4 => LlvmKind::Whitespace,
            5 => LlvmKind::Newline,
            6 => LlvmKind::Error,
            _ => LlvmKind::Eof,
        }
    }

    fn kind_to_raw(kind: Self::Kind) -> u16 {
        match kind {
            LlvmKind::Identifier => 0,
            LlvmKind::Number => 1,
            LlvmKind::String => 2,
            LlvmKind::Comment => 3,
            LlvmKind::Whitespace => 4,
            LlvmKind::Newline => 5,
            LlvmKind::Error => 6,
            LlvmKind::Eof => 7,
        }
    }
}