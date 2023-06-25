use oak_dsv::DsvLanguage;

/// The CSV language configuration.
pub const CSV_LANG: DsvLanguage = DsvLanguage { field_separator: ',', quote_char: '"' };

/// The CSV language marker.
pub type CsvLanguage = oak_dsv::Dsv<CSV_LANG>;
