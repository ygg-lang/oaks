use crate::{kind::TypstSyntaxKind, language::TypstLanguage};
use oak_core::{Lexer, LexerState, SourceText};

type State<'input> = LexerState<'input, TypstLanguage>;

pub struct TypstLexer<'config> {
    config: &'config TypstLanguage,
}

impl<'config> TypstLexer<'config> {
    pub fn new(config: &'config TypstLanguage) -> Self {
        Self { config }
    }
}
