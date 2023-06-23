use crate::{
    language::HandlebarsLanguage,
    lexer::token_type::HandlebarsTokenType,
    parser::{HandlebarsParser, element_type::HandlebarsElementType},
};
use oak_core::{GreenNode, errors::OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, HandlebarsLanguage, S>;

impl<'config> HandlebarsParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, HandlebarsLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let token = match state.current() {
            Some(t) => t,
            None => {
                let err = OakError::unexpected_eof(state.tokens.index(), state.source_id());
                state.errors.push(err.clone());
                return Err(err);
            }
        };

        match token.kind {
            HandlebarsTokenType::Content => {
                let checkpoint = state.checkpoint();
                state.advance();
                state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::ContentNode);
            }
            HandlebarsTokenType::Open => self.parse_mustache(state)?,
            HandlebarsTokenType::OpenUnescaped => self.parse_mustache_unescaped(state)?,
            HandlebarsTokenType::OpenBlock => self.parse_block(state)?,
            HandlebarsTokenType::OpenInverseBlock => self.parse_inverse_block(state)?,
            HandlebarsTokenType::OpenRawBlock => self.parse_raw_block(state)?,
            HandlebarsTokenType::OpenPartial => self.parse_partial(state)?,
            HandlebarsTokenType::OpenComment | HandlebarsTokenType::OpenCommentBlock => self.parse_comment(state)?,
            HandlebarsTokenType::Whitespace | HandlebarsTokenType::Newline => state.advance(),
            _ => {
                // For anything else, treat as error or skip
                state.advance()
            }
        }

        Ok(())
    }

    fn parse_mustache<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::Open)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsTokenType::Close)?;
        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Mustache);
        Ok(())
    }

    fn parse_mustache_unescaped<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::OpenUnescaped)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsTokenType::CloseUnescaped)?;
        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Mustache);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::OpenBlock)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsTokenType::Close)?;

        // Parse block content until closing tag or else
        while state.not_at_end() && !state.at(HandlebarsTokenType::CloseBlock) {
            if state.at(HandlebarsTokenType::Open) {
                // Check if it's an {{else}}
                let next = state.peek_at(1);
                if let Some(token) = next {
                    if token.kind == HandlebarsTokenType::Else {
                        self.parse_else_block(state)?;
                        continue;
                    }
                }
            }
            self.parse_item(state)?
        }

        if state.at(HandlebarsTokenType::CloseBlock) {
            state.expect(HandlebarsTokenType::CloseBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsTokenType::Close)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Block);
        Ok(())
    }

    fn parse_inverse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::OpenInverseBlock)?;

        self.skip_trivia(state);
        if !state.at(HandlebarsTokenType::Close) {
            self.parse_expression(state)?;
            self.skip_trivia(state)
        }

        state.expect(HandlebarsTokenType::Close)?;

        while state.not_at_end() && !state.at(HandlebarsTokenType::CloseBlock) {
            self.parse_item(state)?
        }

        if state.at(HandlebarsTokenType::CloseBlock) {
            state.expect(HandlebarsTokenType::CloseBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsTokenType::Close)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::InverseBlock);
        Ok(())
    }

    fn parse_else_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::Open)?;
        state.expect(HandlebarsTokenType::Else)?;
        state.expect(HandlebarsTokenType::Close)?;

        while state.not_at_end() && !state.at(HandlebarsTokenType::CloseBlock) {
            self.parse_item(state)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::ElseBlock);
        Ok(())
    }

    fn parse_raw_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::OpenRawBlock)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsTokenType::CloseRawBlock)?;

        // In raw blocks, everything is content until the end raw block tag
        while state.not_at_end() && !state.at(HandlebarsTokenType::OpenEndRawBlock) {
            state.advance()
        }

        if state.at(HandlebarsTokenType::OpenEndRawBlock) {
            state.expect(HandlebarsTokenType::OpenEndRawBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsTokenType::CloseRawBlock)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Block);
        Ok(())
    }

    fn parse_partial<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::OpenPartial)?;

        self.skip_trivia(state);
        self.parse_path(state)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(HandlebarsTokenType::Close) {
            self.parse_parameter(state)?;
            self.skip_trivia(state)
        }

        state.expect(HandlebarsTokenType::Close)?;
        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Partial);
        Ok(())
    }

    fn parse_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        if state.at(HandlebarsTokenType::OpenComment) {
            state.expect(HandlebarsTokenType::OpenComment)?;
            while state.not_at_end() && !state.at(HandlebarsTokenType::Close) {
                state.advance()
            }
            state.expect(HandlebarsTokenType::Close)?
        }
        else {
            state.expect(HandlebarsTokenType::OpenCommentBlock)?;
            while state.not_at_end() && !state.at(HandlebarsTokenType::CloseCommentBlock) {
                state.advance()
            }
            state.expect(HandlebarsTokenType::CloseCommentBlock)?
        }
        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::CommentNode);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        self.parse_path(state)?;

        while state.not_at_end() && !state.at(HandlebarsTokenType::Close) && !state.at(HandlebarsTokenType::CloseUnescaped) {
            self.skip_trivia(state);
            if state.at(HandlebarsTokenType::Identifier) || state.at(HandlebarsTokenType::StringLiteral) || state.at(HandlebarsTokenType::NumberLiteral) || state.at(HandlebarsTokenType::BooleanLiteral) { self.parse_parameter(state)? } else { break }
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Expression);
        Ok(())
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(HandlebarsTokenType::Identifier) {
            // Check if it's a named parameter: key=value
            let next = state.peek_at(1);
            if let Some(token) = next {
                if token.kind == HandlebarsTokenType::Equal {
                    let checkpoint = state.checkpoint();
                    state.expect(HandlebarsTokenType::Identifier)?;
                    state.expect(HandlebarsTokenType::Equal)?;
                    self.parse_value(state)?;
                    state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Parameter);
                    return Ok(());
                }
            }
        }

        self.parse_value(state)
    }

    fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(HandlebarsTokenType::Identifier) {
            self.parse_path(state)?
        }
        else if state.at(HandlebarsTokenType::StringLiteral) || state.at(HandlebarsTokenType::NumberLiteral) || state.at(HandlebarsTokenType::BooleanLiteral) {
            state.advance()
        }
        else if state.at(HandlebarsTokenType::LeftParen) {
            self.parse_sub_expression(state)?
        }
        else {
            let token = state.current();
            let err = oak_core::errors::OakError::unexpected_token(format!("{:?}", token.map(|t| t.kind)), state.tokens.index(), state.source_id());
            state.errors.push(err.clone());
            return Err(err);
        }
        Ok(())
    }

    fn parse_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::Identifier)?;

        while state.at(HandlebarsTokenType::Dot) || state.at(HandlebarsTokenType::Slash) {
            state.advance();
            state.expect(HandlebarsTokenType::Identifier)?
        }

        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::Path);
        Ok(())
    }

    fn parse_sub_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsTokenType::LeftParen)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsTokenType::RightParen)?;
        state.finish_at(checkpoint, crate::parser::element_type::HandlebarsElementType::SubExpression);
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(HandlebarsTokenType::Whitespace) || state.at(HandlebarsTokenType::Newline) {
            state.advance()
        }
    }
}
