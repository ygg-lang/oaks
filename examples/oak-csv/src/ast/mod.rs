use crate::language::CSV_LANG;

/// A CSV root.
pub type CsvRoot = oak_dsv::ast::DsvRoot<CSV_LANG>;
/// A CSV record.
pub type CsvRecord = oak_dsv::ast::DsvRecord<CSV_LANG>;
/// A CSV field.
pub type CsvField = oak_dsv::ast::DsvField<CSV_LANG>;
