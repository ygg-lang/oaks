use oak_dsv::DsvLanguage;

/// The TSV language configuration.
pub const TSV_LANG: DsvLanguage = DsvLanguage { field_separator: '\t', quote_char: '"' };

/// The TSV language marker.
pub type TsvLanguage = oak_dsv::Dsv<TSV_LANG>;
