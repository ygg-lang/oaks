pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

pub use crate::{ast::WatRoot, kind::WatSyntaxKind, language::WatLanguage, lexer::WatLexer, parser::WatParser};
