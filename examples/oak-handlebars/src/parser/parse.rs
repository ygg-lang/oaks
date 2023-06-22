use crate::{kind::HandlebarsSyntaxKind, language::HandlebarsLanguage, parser::HandlebarsParser};
use oak_core::{GreenNode, errors::OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, HandlebarsLanguage, S>;

impl<'config> HandlebarsParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, HandlebarsLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?;
        }

        Ok(state.finish_at(checkpoint, HandlebarsSyntaxKind::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let token = match state.current() {
            Some(t) => t,
            None => {
                let err = oak_core::errors::OakError::unexpected_eof(state.tokens.index(), state.source_url());
                state.errors.push(err.clone());
                return Err(err);
            }
        };

        match token.kind {
            HandlebarsSyntaxKind::Content => {
                let checkpoint = state.checkpoint();
                state.advance();
                state.finish_at(checkpoint, HandlebarsSyntaxKind::ContentNode);
            }
            HandlebarsSyntaxKind::Open => {
                self.parse_mustache(state)?;
            }
            HandlebarsSyntaxKind::OpenUnescaped => {
                self.parse_mustache_unescaped(state)?;
            }
            HandlebarsSyntaxKind::OpenBlock => {
                self.parse_block(state)?;
            }
            HandlebarsSyntaxKind::OpenInverseBlock => {
                self.parse_inverse_block(state)?;
            }
            HandlebarsSyntaxKind::OpenRawBlock => {
                self.parse_raw_block(state)?;
            }
            HandlebarsSyntaxKind::OpenPartial => {
                self.parse_partial(state)?;
            }
            HandlebarsSyntaxKind::OpenComment | HandlebarsSyntaxKind::OpenCommentBlock => {
                self.parse_comment(state)?;
            }
            HandlebarsSyntaxKind::Whitespace | HandlebarsSyntaxKind::Newline => {
                state.advance();
            }
            _ => {
                // For anything else, treat as error or skip
                state.advance();
            }
        }

        Ok(())
    }

    fn parse_mustache<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::Open)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsSyntaxKind::Close)?;
        state.finish_at(checkpoint, HandlebarsSyntaxKind::Mustache);
        Ok(())
    }

    fn parse_mustache_unescaped<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::OpenUnescaped)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsSyntaxKind::CloseUnescaped)?;
        state.finish_at(checkpoint, HandlebarsSyntaxKind::Mustache);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::OpenBlock)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsSyntaxKind::Close)?;

        // Parse block content until closing tag or else
        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::CloseBlock) {
            if state.at(HandlebarsSyntaxKind::Open) {
                // Check if it's an {{else}}
                let next = state.peek_at(1);
                if let Some(token) = next {
                    if token.kind == HandlebarsSyntaxKind::Else {
                        self.parse_else_block(state)?;
                        continue;
                    }
                }
            }
            self.parse_item(state)?;
        }

        if state.at(HandlebarsSyntaxKind::CloseBlock) {
            state.expect(HandlebarsSyntaxKind::CloseBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsSyntaxKind::Close)?;
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::Block);
        Ok(())
    }

    fn parse_inverse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::OpenInverseBlock)?;

        self.skip_trivia(state);
        if !state.at(HandlebarsSyntaxKind::Close) {
            self.parse_expression(state)?;
            self.skip_trivia(state);
        }

        state.expect(HandlebarsSyntaxKind::Close)?;

        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::CloseBlock) {
            self.parse_item(state)?;
        }

        if state.at(HandlebarsSyntaxKind::CloseBlock) {
            state.expect(HandlebarsSyntaxKind::CloseBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsSyntaxKind::Close)?;
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::InverseBlock);
        Ok(())
    }

    fn parse_else_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::Open)?;
        state.expect(HandlebarsSyntaxKind::Else)?;
        state.expect(HandlebarsSyntaxKind::Close)?;

        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::CloseBlock) {
            self.parse_item(state)?;
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::ElseBlock);
        Ok(())
    }

    fn parse_raw_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::OpenRawBlock)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsSyntaxKind::CloseRawBlock)?;

        // In raw blocks, everything is content until the end raw block tag
        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::OpenEndRawBlock) {
            state.advance();
        }

        if state.at(HandlebarsSyntaxKind::OpenEndRawBlock) {
            state.expect(HandlebarsSyntaxKind::OpenEndRawBlock)?;
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.expect(HandlebarsSyntaxKind::CloseRawBlock)?;
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::Block);
        Ok(())
    }

    fn parse_partial<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::OpenPartial)?;

        self.skip_trivia(state);
        self.parse_path(state)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::Close) {
            self.parse_parameter(state)?;
            self.skip_trivia(state);
        }

        state.expect(HandlebarsSyntaxKind::Close)?;
        state.finish_at(checkpoint, HandlebarsSyntaxKind::Partial);
        Ok(())
    }

    fn parse_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        if state.at(HandlebarsSyntaxKind::OpenComment) {
            state.expect(HandlebarsSyntaxKind::OpenComment)?;
            while state.not_at_end() && !state.at(HandlebarsSyntaxKind::Close) {
                state.advance();
            }
            state.expect(HandlebarsSyntaxKind::Close)?;
        }
        else {
            state.expect(HandlebarsSyntaxKind::OpenCommentBlock)?;
            while state.not_at_end() && !state.at(HandlebarsSyntaxKind::CloseCommentBlock) {
                state.advance();
            }
            state.expect(HandlebarsSyntaxKind::CloseCommentBlock)?;
        }
        state.finish_at(checkpoint, HandlebarsSyntaxKind::CommentNode);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        self.parse_path(state)?;

        while state.not_at_end() && !state.at(HandlebarsSyntaxKind::Close) && !state.at(HandlebarsSyntaxKind::CloseUnescaped) {
            self.skip_trivia(state);
            if state.at(HandlebarsSyntaxKind::Identifier) || state.at(HandlebarsSyntaxKind::StringLiteral) || state.at(HandlebarsSyntaxKind::NumberLiteral) || state.at(HandlebarsSyntaxKind::BooleanLiteral) {
                self.parse_parameter(state)?;
            }
            else {
                break;
            }
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::Expression);
        Ok(())
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(HandlebarsSyntaxKind::Identifier) {
            // Check if it's a named parameter: key=value
            let next = state.peek_at(1);
            if let Some(token) = next {
                if token.kind == HandlebarsSyntaxKind::Equal {
                    let checkpoint = state.checkpoint();
                    state.expect(HandlebarsSyntaxKind::Identifier)?;
                    state.expect(HandlebarsSyntaxKind::Equal)?;
                    self.parse_value(state)?;
                    state.finish_at(checkpoint, HandlebarsSyntaxKind::Parameter);
                    return Ok(());
                }
            }
        }

        self.parse_value(state)
    }

    fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(HandlebarsSyntaxKind::Identifier) {
            self.parse_path(state)?;
        }
        else if state.at(HandlebarsSyntaxKind::StringLiteral) || state.at(HandlebarsSyntaxKind::NumberLiteral) || state.at(HandlebarsSyntaxKind::BooleanLiteral) {
            state.advance();
        }
        else if state.at(HandlebarsSyntaxKind::LeftParen) {
            self.parse_sub_expression(state)?;
        }
        else {
            let token = state.current();
            let err = oak_core::errors::OakError::unexpected_token(format!("{:?}", token.map(|t| t.kind)), state.tokens.index(), state.source_url());
            state.errors.push(err.clone());
            return Err(err);
        }
        Ok(())
    }

    fn parse_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::Identifier)?;

        while state.at(HandlebarsSyntaxKind::Dot) || state.at(HandlebarsSyntaxKind::Slash) {
            state.advance();
            state.expect(HandlebarsSyntaxKind::Identifier)?;
        }

        state.finish_at(checkpoint, HandlebarsSyntaxKind::Path);
        Ok(())
    }

    fn parse_sub_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(HandlebarsSyntaxKind::LeftParen)?;

        self.skip_trivia(state);
        self.parse_expression(state)?;
        self.skip_trivia(state);

        state.expect(HandlebarsSyntaxKind::RightParen)?;
        state.finish_at(checkpoint, HandlebarsSyntaxKind::SubExpression);
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(HandlebarsSyntaxKind::Whitespace) || state.at(HandlebarsSyntaxKind::Newline) {
            state.advance();
        }
    }
}
