/// Element types and tree structure definitions for Dockerfile.
pub mod element_type;

use crate::{language::DockerfileLanguage, lexer::DockerfileLexer, parser::element_type::DockerfileElementType};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser for Dockerfile.
pub struct DockerfileParser<'config> {
    pub(crate) config: &'config DockerfileLanguage,
}

impl<'config> DockerfileParser<'config> {
    /// Creates a new `DockerfileParser` with the given language configuration.
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DockerfileLanguage> for DockerfileParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DockerfileLanguage>) -> ParseOutput<'a, DockerfileLanguage> {
        let lexer = DockerfileLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let root_checkpoint = state.checkpoint();
            while state.not_at_end() {
                if state.at(crate::lexer::token_type::DockerfileTokenType::Whitespace) || state.at(crate::lexer::token_type::DockerfileTokenType::Newline) {
                    state.advance();
                    continue;
                }

                if state.at(crate::lexer::token_type::DockerfileTokenType::Comment) {
                    let checkpoint = state.checkpoint();
                    state.advance();
                    state.finish_at(checkpoint, DockerfileElementType::Comment);
                    continue;
                }

                // Parse instructions
                let instruction_checkpoint = state.checkpoint();
                let kind = match state.peek_kind() {
                    Some(crate::lexer::token_type::DockerfileTokenType::From) => Some(DockerfileElementType::From),
                    Some(crate::lexer::token_type::DockerfileTokenType::Run) => Some(DockerfileElementType::Run),
                    Some(crate::lexer::token_type::DockerfileTokenType::Cmd) => Some(DockerfileElementType::Cmd),
                    Some(crate::lexer::token_type::DockerfileTokenType::Label) => Some(DockerfileElementType::Label),
                    Some(crate::lexer::token_type::DockerfileTokenType::Expose) => Some(DockerfileElementType::Expose),
                    Some(crate::lexer::token_type::DockerfileTokenType::Env) => Some(DockerfileElementType::Env),
                    Some(crate::lexer::token_type::DockerfileTokenType::Add) => Some(DockerfileElementType::Add),
                    Some(crate::lexer::token_type::DockerfileTokenType::Copy) => Some(DockerfileElementType::Copy),
                    Some(crate::lexer::token_type::DockerfileTokenType::Entrypoint) => Some(DockerfileElementType::Entrypoint),
                    Some(crate::lexer::token_type::DockerfileTokenType::Volume) => Some(DockerfileElementType::Volume),
                    Some(crate::lexer::token_type::DockerfileTokenType::User) => Some(DockerfileElementType::User),
                    Some(crate::lexer::token_type::DockerfileTokenType::Workdir) => Some(DockerfileElementType::Workdir),
                    Some(crate::lexer::token_type::DockerfileTokenType::Arg) => Some(DockerfileElementType::Arg),
                    Some(crate::lexer::token_type::DockerfileTokenType::Onbuild) => Some(DockerfileElementType::Onbuild),
                    Some(crate::lexer::token_type::DockerfileTokenType::Stopsignal) => Some(DockerfileElementType::Stopsignal),
                    Some(crate::lexer::token_type::DockerfileTokenType::Healthcheck) => Some(DockerfileElementType::Healthcheck),
                    Some(crate::lexer::token_type::DockerfileTokenType::Shell) => Some(DockerfileElementType::Shell),
                    Some(crate::lexer::token_type::DockerfileTokenType::Maintainer) => Some(DockerfileElementType::Maintainer),
                    _ => None,
                };

                if let Some(kind) = kind {
                    state.advance(); // consume instruction keyword
                    // Consume until newline or EOF
                    while state.not_at_end() && !state.at(crate::lexer::token_type::DockerfileTokenType::Newline) {
                        state.advance();
                    }
                    state.finish_at(instruction_checkpoint, kind);
                }
                else {
                    // Unknown instruction or something else
                    state.advance();
                }
            }
            Ok(state.finish_at(root_checkpoint, DockerfileElementType::Root))
        })
    }
}
