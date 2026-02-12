use crate::language::DsvLanguage;
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};

/// The root node of a DSV document.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DsvRoot<const LANG: DsvLanguage> {
    /// The records in the DSV document.
    pub records: Vec<DsvRecord<LANG>>,
}

impl<const LANG: DsvLanguage> ToSource for DsvRoot<LANG> {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, record) in self.records.iter().enumerate() {
            if i > 0 {
                buffer.push("\n");
            }
            record.to_source(buffer);
        }
    }
}

impl<const LANG: DsvLanguage> DsvRoot<LANG> {
    /// Creates a new `DsvRoot` with the given records.
    pub fn new(records: Vec<DsvRecord<LANG>>) -> Self {
        Self { records }
    }
}

/// Represents a single record in a DSV document.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DsvRecord<const LANG: DsvLanguage> {
    /// The fields in the record.
    pub fields: Vec<DsvField<LANG>>,
    /// The source range of the record.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl<const LANG: DsvLanguage> ToSource for DsvRecord<LANG> {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                buffer.push_char(LANG.field_separator);
            }
            field.to_source(buffer);
        }
    }
}

/// Represents a single field in a DSV record.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DsvField<const LANG: DsvLanguage> {
    /// The value of the field.
    pub value: String,
    /// Whether the field was quoted in the source.
    pub is_quoted: bool,
    /// The source range of the field.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl<const LANG: DsvLanguage> ToSource for DsvField<LANG> {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        if self.is_quoted {
            buffer.push_char(LANG.quote_char);
            let escaped = self.value.replace(LANG.quote_char, &format!("{}{}", LANG.quote_char, LANG.quote_char));
            buffer.push(&escaped);
            buffer.push_char(LANG.quote_char);
        }
        else {
            buffer.push(&self.value);
        }
    }
}
