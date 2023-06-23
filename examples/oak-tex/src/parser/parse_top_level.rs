use crate::{
    language::TexLanguage,
    lexer::token_type::TexTokenType,
    parser::{State, TexParser, element_type::TexElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TexParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TexLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::TexElementType::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(TexTokenType::BeginKeyword) => self.parse_environment(state),
            Some(kind) if self.is_command_like(kind) => self.parse_command(state),
            Some(TexTokenType::LeftBrace) => self.parse_group(state),
            Some(TexTokenType::Dollar) | Some(TexTokenType::DoubleDollar) => self.parse_math(state),
            Some(TexTokenType::Caret) => self.parse_superscript(state),
            Some(TexTokenType::Underscore) => self.parse_subscript(state),
            _ => {
                state.bump();
                Ok(())
            }
        }
    }

    fn is_command_like(&self, kind: TexTokenType) -> bool {
        match kind {
            TexTokenType::Backslash |
            TexTokenType::Command |
            // TexTokenType::BeginKeyword | // Handled separately
            TexTokenType::EndKeyword |
            TexTokenType::DocumentclassKeyword |
            TexTokenType::UsepackageKeyword |
            TexTokenType::SectionKeyword |
            TexTokenType::SubsectionKeyword |
            TexTokenType::SubsubsectionKeyword |
            TexTokenType::ChapterKeyword |
            TexTokenType::PartKeyword |
            TexTokenType::TitleKeyword |
            TexTokenType::AuthorKeyword |
            TexTokenType::DateKeyword |
            TexTokenType::MaketitleKeyword |
            TexTokenType::TableofcontentsKeyword |
            TexTokenType::ItemKeyword |
            TexTokenType::LabelKeyword |
            TexTokenType::RefKeyword |
            TexTokenType::CiteKeyword |
            TexTokenType::IncludegraphicsKeyword |
            TexTokenType::TextbfKeyword |
            TexTokenType::TextitKeyword |
            TexTokenType::EmphKeyword |
            TexTokenType::Frac |
            TexTokenType::Sqrt |
            TexTokenType::Sum |
            TexTokenType::Int |
            TexTokenType::Lim |
            TexTokenType::Alpha |
            TexTokenType::Beta |
            TexTokenType::Gamma |
            TexTokenType::Delta |
            TexTokenType::Epsilon => true,
            _ => false,
        }
    }

    fn parse_environment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Parse \begin{name}
        let begin_checkpoint = state.checkpoint();
        state.expect(TexTokenType::BeginKeyword)?;
        if state.at(TexTokenType::LeftBrace) {
            self.parse_mandatory_argument(state)?;
        }
        state.finish_at(begin_checkpoint, crate::parser::element_type::TexElementType::BeginEnvironment);

        // Parse content until \end{name}
        while state.not_at_end() && !state.at(TexTokenType::EndKeyword) {
            self.parse_item(state)?
        }

        // Parse \end{name}
        if state.at(TexTokenType::EndKeyword) {
            let end_checkpoint = state.checkpoint();
            state.bump();
            if state.at(TexTokenType::LeftBrace) {
                self.parse_mandatory_argument(state)?
            }
            state.finish_at(end_checkpoint, crate::parser::element_type::TexElementType::EndEnvironment);
        }

        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::Environment);
        Ok(())
    }

    fn parse_superscript<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexTokenType::Caret)?;

        if state.at(TexTokenType::LeftBrace) {
            self.parse_group(state)?;
        }
        else {
            state.bump();
        }

        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::Superscript);
        Ok(())
    }

    fn parse_subscript<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexTokenType::Underscore)?;

        if state.at(TexTokenType::LeftBrace) {
            self.parse_group(state)?;
        }
        else {
            state.bump();
        }

        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::Subscript);
        Ok(())
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().unwrap_or(TexTokenType::Command);

        let should_parse_args = state.peek_text().map_or(true, |name| {
            let name_str: &str = &name;
            let name_str = name_str.strip_prefix('\\').unwrap_or(name_str);
            name_str != "left" && name_str != "right"
        });

        state.bump(); // consume \ or command keyword

        let node_kind = if should_parse_args {
            while state.at(TexTokenType::LeftBracket) || state.at(TexTokenType::LeftBrace) {
                if state.at(TexTokenType::LeftBracket) {
                    self.parse_optional_argument(state)?;
                }
                else {
                    self.parse_mandatory_argument(state)?;
                }
            }
            match kind {
                TexTokenType::Frac | TexTokenType::Sqrt | TexTokenType::Sum | TexTokenType::Int | TexTokenType::Lim | TexTokenType::Alpha | TexTokenType::Beta | TexTokenType::Gamma | TexTokenType::Delta | TexTokenType::Epsilon => kind,
                _ => TexTokenType::Command,
            }
        }
        else {
            kind
        };

        state.finish_at(checkpoint, node_kind.into());
        Ok(())
    }

    fn parse_group<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexTokenType::LeftBrace)?;
        while state.not_at_end() && !state.at(TexTokenType::RightBrace) {
            self.parse_item(state)?;
        }
        state.expect(TexTokenType::RightBrace)?;
        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::Group);
        Ok(())
    }

    fn parse_math<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().unwrap();
        state.bump();
        while state.not_at_end() && !state.at(kind) {
            self.parse_item(state)?;
        }
        state.expect(kind)?;
        let element_kind = if kind == TexTokenType::DoubleDollar { crate::parser::element_type::TexElementType::DisplayMath } else { crate::parser::element_type::TexElementType::InlineMath };
        state.finish_at(checkpoint, element_kind);
        Ok(())
    }

    fn parse_optional_argument<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexTokenType::LeftBracket)?;
        while state.not_at_end() && !state.at(TexTokenType::RightBracket) {
            self.parse_item(state)?;
        }
        state.expect(TexTokenType::RightBracket)?;
        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::OptionalArgument);
        Ok(())
    }

    fn parse_mandatory_argument<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexTokenType::LeftBrace)?;
        while state.not_at_end() && !state.at(TexTokenType::RightBrace) {
            self.parse_item(state)?;
        }
        state.expect(TexTokenType::RightBrace)?;
        state.finish_at(checkpoint, crate::parser::element_type::TexElementType::MandatoryArgument);
        Ok(())
    }
}
