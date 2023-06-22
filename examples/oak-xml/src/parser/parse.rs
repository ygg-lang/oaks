use crate::{XmlLanguage, kind::XmlSyntaxKind, parser::XmlParser};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, XmlLanguage, S>;

impl<'config> XmlParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, XmlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        self.skip_trivia(state);
        if state.at(XmlSyntaxKind::LeftAngle) && state.peek_kind_at(1) == Some(XmlSyntaxKind::Question) {
            self.parse_prolog(state)?;
        }

        while state.not_at_end() {
            self.skip_trivia(state);
            if state.at(XmlSyntaxKind::LeftAngle) {
                self.parse_element(state)?;
            }
            else if state.not_at_end() {
                state.advance();
            }
        }

        Ok(state.finish_at(checkpoint, XmlSyntaxKind::Root))
    }

    pub(crate) fn parse_prolog<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(XmlSyntaxKind::LeftAngle)?;
        state.expect(XmlSyntaxKind::Question)?;
        state.expect(XmlSyntaxKind::Identifier)?; // xml

        while state.not_at_end() && !state.at(XmlSyntaxKind::Question) {
            self.skip_trivia(state);
            if state.at(XmlSyntaxKind::Identifier) {
                self.parse_attribute(state)?;
            }
            else {
                break;
            }
        }

        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::Question)?;
        state.expect(XmlSyntaxKind::RightAngle)?;
        state.finish_at(checkpoint, XmlSyntaxKind::Prolog);
        Ok(())
    }

    pub(crate) fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Start Tag
        let start_tag_checkpoint = state.checkpoint();
        state.expect(XmlSyntaxKind::LeftAngle)?;
        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::Identifier)?;

        while state.not_at_end() && !state.at(XmlSyntaxKind::RightAngle) && !state.at(XmlSyntaxKind::SlashRightAngle) {
            self.skip_trivia(state);
            if state.at(XmlSyntaxKind::Identifier) {
                self.parse_attribute(state)?;
            }
            else {
                break;
            }
        }

        self.skip_trivia(state);
        let is_self_closing = state.at(XmlSyntaxKind::SlashRightAngle);
        if is_self_closing {
            let self_closing_checkpoint = state.checkpoint();
            state.expect(XmlSyntaxKind::SlashRightAngle)?;
            state.finish_at(self_closing_checkpoint, XmlSyntaxKind::SelfClosingTag);
            state.finish_at(checkpoint, XmlSyntaxKind::Element);
            return Ok(());
        }

        state.expect(XmlSyntaxKind::RightAngle)?;
        state.finish_at(start_tag_checkpoint, XmlSyntaxKind::StartTag);

        // Content
        while state.not_at_end() {
            if state.at(XmlSyntaxKind::LeftAngleSlash) {
                break;
            }
            if state.at(XmlSyntaxKind::LeftAngle) {
                self.parse_element(state)?;
            }
            else {
                state.advance();
            }
        }

        // End Tag
        let end_tag_checkpoint = state.checkpoint();
        state.expect(XmlSyntaxKind::LeftAngleSlash)?;
        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::Identifier)?;
        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::RightAngle)?;
        state.finish_at(end_tag_checkpoint, XmlSyntaxKind::EndTag);

        state.finish_at(checkpoint, XmlSyntaxKind::Element);
        Ok(())
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(XmlSyntaxKind::Identifier)?;
        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::Equals)?;
        self.skip_trivia(state);
        state.expect(XmlSyntaxKind::StringLiteral)?;
        state.finish_at(checkpoint, XmlSyntaxKind::Attribute);
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use oak_core::TokenType;
        while let Some(token) = state.current() {
            if token.kind.is_ignored() {
                state.bump();
            }
            else {
                break;
            }
        }
    }
}
