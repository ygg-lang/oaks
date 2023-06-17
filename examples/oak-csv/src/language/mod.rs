use crate::kind::CsvSyntaxKind;
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CsvLanguage;

impl Language for CsvLanguage {
    type SyntaxKind = CsvSyntaxKind;
}
