use oak_core::Language;

pub struct IdlLanguage;

impl Language for IdlLanguage {
    type Kind = crate::kind::IdlSyntaxKind;
}
