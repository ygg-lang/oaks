use crate::language::CSV_LANG;
/// A CSV parser.
pub type CsvParser = oak_dsv::parser::DsvParser<CSV_LANG>;
