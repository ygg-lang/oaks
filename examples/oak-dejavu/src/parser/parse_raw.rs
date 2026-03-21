use crate::{lexer::token_type::DejavuTokenType, parser::element_type::DejavuElementType::*};
use oak_core::{GreenNode, OakError, source::Source};

use super::State;

impl<'config> super::DejavuParser<'config> {
    /// Parse a raw block: `<% raw %>原始文本<% end raw %>`
    pub(crate) fn parse_raw<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        // Consume the opening `<%`
        self.skip_trivia(state);
        if state.at(DejavuTokenType::TemplateControlStart) {
            state.expect(DejavuTokenType::TemplateControlStart)?;
        }
        else if state.at(DejavuTokenType::CodeStart) {
            state.expect(DejavuTokenType::CodeStart)?;
        }
        else {
            return Err(OakError::custom_error("Expected TemplateControlStart or CodeStart"));
        }
        self.skip_trivia(state);

        // Consume the `raw` keyword
        state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Raw))?;
        self.skip_trivia(state);

        // Consume the closing `%>`
        if state.at(DejavuTokenType::TemplateControlEnd) {
            state.expect(DejavuTokenType::TemplateControlEnd)?;
        }
        else if state.at(DejavuTokenType::CodeEnd) {
            state.expect(DejavuTokenType::CodeEnd)?;
        }
        else {
            return Err(OakError::custom_error("Expected TemplateControlEnd or CodeEnd"));
        }

        // Parse the raw content until we find the closing tag
        while state.not_at_end() {
            // Check if we've reached the closing tag `<% end raw %>`
            let checkpoint = state.checkpoint();
            if self.skip_trivia_and_check_closing_tag(state) {
                // Consume the closing tag
                if state.at(DejavuTokenType::TemplateControlStart) {
                    state.expect(DejavuTokenType::TemplateControlStart)?;
                }
                else if state.at(DejavuTokenType::CodeStart) {
                    state.expect(DejavuTokenType::CodeStart)?;
                }
                self.skip_trivia(state);
                state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End))?;
                self.skip_trivia(state);
                state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Raw))?;
                self.skip_trivia(state);
                if state.at(DejavuTokenType::TemplateControlEnd) {
                    state.expect(DejavuTokenType::TemplateControlEnd)?;
                }
                else if state.at(DejavuTokenType::CodeEnd) {
                    state.expect(DejavuTokenType::CodeEnd)?;
                }
                break;
            }
            else {
                // Restore the checkpoint and consume the current token
                state.restore(checkpoint);
                state.bump();
            }
        }

        Ok(state.finish_at(cp, RawBlockNode))
    }

    /// Skip trivia and check if we've reached the closing tag `<% end raw %>`
    fn skip_trivia_and_check_closing_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let checkpoint = state.checkpoint();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `<%`
        if !state.at(DejavuTokenType::TemplateControlStart) && !state.at(DejavuTokenType::CodeStart) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `end` keyword
        if !state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End)) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `raw` keyword
        if !state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Raw)) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `%>`
        if !state.at(DejavuTokenType::TemplateControlEnd) && !state.at(DejavuTokenType::CodeEnd) {
            state.restore(checkpoint);
            return false;
        }

        state.restore(checkpoint);
        return true;
    }

    /// Parse a block: `<% block "block_name" %>内容<% end block %>`
    pub(crate) fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        // Consume the opening `<%`
        self.skip_trivia(state);
        if state.at(DejavuTokenType::TemplateControlStart) {
            state.expect(DejavuTokenType::TemplateControlStart)?;
        }
        else if state.at(DejavuTokenType::CodeStart) {
            state.expect(DejavuTokenType::CodeStart)?;
        }
        else {
            return Err(OakError::custom_error("Expected TemplateControlStart or CodeStart"));
        }
        self.skip_trivia(state);

        // Consume the `block` keyword
        state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Block))?;
        self.skip_trivia(state);

        // Parse block content
        let block_node = self.parse_block_content(state)?;

        Ok(state.finish_at(cp, BlockDeclaration))
    }

    /// Parse block content after the `block` keyword
    pub(crate) fn parse_block_content<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        // Consume the block name (string literal)
        if !state.at(DejavuTokenType::StringLiteral) {
            return Err(OakError::custom_error(format!("Expected string literal for block name, found {:?}", state.current().map(|t| t.kind))));
        }
        state.bump();
        self.skip_trivia(state);

        // Consume the closing `%>`
        if state.at(DejavuTokenType::TemplateControlEnd) {
            state.expect(DejavuTokenType::TemplateControlEnd)?;
        }
        else if state.at(DejavuTokenType::CodeEnd) {
            state.expect(DejavuTokenType::CodeEnd)?;
        }
        else {
            return Err(OakError::custom_error("Expected TemplateControlEnd or CodeEnd"));
        }

        // Parse the block content until we find the closing tag
        while state.not_at_end() {
            // Check if we've reached the closing tag `<% end block %>`
            let checkpoint = state.checkpoint();
            if self.skip_trivia_and_check_closing_block_tag(state) {
                // Consume the closing tag
                if state.at(DejavuTokenType::TemplateControlStart) {
                    state.expect(DejavuTokenType::TemplateControlStart)?;
                }
                else if state.at(DejavuTokenType::CodeStart) {
                    state.expect(DejavuTokenType::CodeStart)?;
                }
                self.skip_trivia(state);
                state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End))?;
                self.skip_trivia(state);
                state.expect(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Block))?;
                self.skip_trivia(state);
                if state.at(DejavuTokenType::TemplateControlEnd) {
                    state.expect(DejavuTokenType::TemplateControlEnd)?;
                }
                else if state.at(DejavuTokenType::CodeEnd) {
                    state.expect(DejavuTokenType::CodeEnd)?;
                }
                break;
            }
            else {
                // Restore the checkpoint and consume the current token
                state.restore(checkpoint);
                state.bump();
            }
        }

        Ok(state.finish_at(cp, BlockDeclaration))
    }

    /// Skip trivia and check if we've reached the closing tag `<% end block %>`
    fn skip_trivia_and_check_closing_block_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let checkpoint = state.checkpoint();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `<%`
        if !state.at(DejavuTokenType::TemplateControlStart) && !state.at(DejavuTokenType::CodeStart) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `end` keyword
        if !state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End)) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `block` keyword
        if !state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Block)) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        // Skip any whitespace
        while state.at(DejavuTokenType::Whitespace) {
            state.bump();
        }

        // Check for `%>`
        if !state.at(DejavuTokenType::TemplateControlEnd) && !state.at(DejavuTokenType::CodeEnd) {
            state.restore(checkpoint);
            return false;
        }

        state.restore(checkpoint);
        return true;
    }
}

/// Extension trait for DejavuParser
pub(crate) trait DejavuParserExt {
    fn parse_template_control<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError>;
}

impl<'config> DejavuParserExt for super::DejavuParser<'config> {
    fn parse_template_control<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        // Consume the opening `<%` or CodeStart
        self.skip_trivia(state);
        if state.at(DejavuTokenType::TemplateControlStart) {
            state.expect(DejavuTokenType::TemplateControlStart)?;
        }
        else if state.at(DejavuTokenType::CodeStart) {
            state.expect(DejavuTokenType::CodeStart)?;
        }
        else {
            return Err(OakError::custom_error("Expected TemplateControlStart or CodeStart"));
        }
        self.skip_trivia(state);

        // Parse the directive
        let directive_node = if let Some(token) = state.current() {
            match token.kind {
                DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Include) => self.parse_include(state)?,
                DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Raw) => {
                    // Handle raw separately as it has a different structure
                    state.restore(cp);
                    return self.parse_raw(state);
                }
                DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Block) => {
                    // Handle block separately as it has a different structure
                    state.bump(); // Consume the block keyword
                    self.skip_trivia(state);
                    let block_node = self.parse_block_content(state)?;
                    return Ok(block_node);
                }
                _ => {
                    // For all other directives, just consume tokens until we reach %> or CodeEnd
                    // and create a TemplateControl node
                    while state.not_at_end() && !state.at(DejavuTokenType::TemplateControlEnd) && !state.at(DejavuTokenType::CodeEnd) {
                        state.bump();
                    }
                    state.finish_at(cp, TemplateControl)
                }
            }
        }
        else {
            return Err(OakError::custom_error("Unexpected end of file in template control directive"));
        };

        // Consume the closing `%>` or CodeEnd if not already consumed
        self.skip_trivia(state);
        if state.at(DejavuTokenType::TemplateControlEnd) {
            state.expect(DejavuTokenType::TemplateControlEnd)?;
        }
        else if state.at(DejavuTokenType::CodeEnd) {
            state.expect(DejavuTokenType::CodeEnd)?;
        }

        Ok(directive_node)
    }
}
