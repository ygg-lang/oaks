use crate::{
    language::VocLanguage,
    lexer::token_type::VocTokenType,
    parser::{State, VocElementType, VocParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VocParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        match self.config.mode {
            crate::language::VocMode::Programming => {
                self.parse_programming_root(state)?;
            }
            crate::language::VocMode::Component => {
                self.parse_component_root(state)?;
            }
        }

        Ok(state.finish_at(checkpoint, VocElementType::SourceFile))
    }

    fn parse_programming_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(VocTokenType::Eof) {
            // For now, just consume tokens.
            // In a real implementation, this would look like Valkyrie's parse_source_file.
            state.bump();
        }
        Ok(())
    }

    fn parse_component_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(VocTokenType::Eof) {
            if state.at(VocTokenType::TagOpen) {
                self.parse_tag(state)?;
            }
            else if state.at(VocTokenType::InterpolationStart) {
                self.parse_interpolation(state)?;
            }
            else if state.at(VocTokenType::TemplateControlStart) {
                self.parse_template_control(state)?;
            }
            else {
                state.bump(); // TextPart or others
            }
        }
        Ok(())
    }

    fn parse_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(VocTokenType::TagOpen)?;

        if state.at(VocTokenType::TagSlash) {
            state.bump();
        }

        // Tag name
        if state.at(VocTokenType::Identifier) || matches!(state.peek_kind(), Some(VocTokenType::Keyword(_))) {
            state.bump();
        }

        // Attributes
        while state.not_at_end() && !state.at(VocTokenType::TagClose) && !state.at(VocTokenType::TagSelfClose) {
            if state.at(VocTokenType::Identifier) {
                self.parse_attribute(state)?;
            }
            else {
                state.bump();
            }
        }

        if state.at(VocTokenType::TagSelfClose) {
            state.bump();
        }
        else {
            state.expect(VocTokenType::TagClose)?;
        }

        Ok(state.finish_at(cp, VocElementType::Tag))
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(VocTokenType::Identifier)?;

        if state.at(VocTokenType::AttrEq) {
            state.bump();
            if state.at(VocTokenType::StringLiteral) {
                state.bump();
            }
        }

        Ok(state.finish_at(cp, VocElementType::Attribute))
    }

    fn parse_interpolation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(VocTokenType::InterpolationStart)?;

        // In a real implementation, we would parse an expression here
        while state.not_at_end() && !state.at(VocTokenType::InterpolationEnd) {
            state.bump();
        }

        state.expect(VocTokenType::InterpolationEnd)?;
        Ok(state.finish_at(cp, VocElementType::Interpolation))
    }

    fn parse_template_control<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(VocTokenType::TemplateControlStart)?;

        while state.not_at_end() && !state.at(VocTokenType::TemplateControlEnd) {
            state.bump();
        }

        state.expect(VocTokenType::TemplateControlEnd)?;
        Ok(state.finish_at(cp, VocElementType::TemplateControl))
    }
}
