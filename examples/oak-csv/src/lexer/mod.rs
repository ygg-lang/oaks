use crate::language::CSV_LANG;
/// A CSV lexer.
pub type CsvLexer = oak_dsv::lexer::DsvLexer<CSV_LANG>;
