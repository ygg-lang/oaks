use crate::language::TSV_LANG;

/// A TSV root.
pub type TsvRoot = oak_dsv::ast::DsvRoot<TSV_LANG>;
/// A TSV record.
pub type TsvRecord = oak_dsv::ast::DsvRecord<TSV_LANG>;
/// A TSV field.
pub type TsvField = oak_dsv::ast::DsvField<TSV_LANG>;
