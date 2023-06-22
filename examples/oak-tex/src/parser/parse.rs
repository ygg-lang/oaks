use crate::{
    kind::TexSyntaxKind,
    language::TexLanguage,
    parser::{State, TexParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TexParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TexLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?;
        }

        Ok(state.finish_at(checkpoint, TexSyntaxKind::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(TexSyntaxKind::BeginKeyword) => self.parse_environment(state),
            Some(kind) if self.is_command_like(kind) => self.parse_command(state),
            Some(TexSyntaxKind::LeftBrace) => self.parse_group(state),
            Some(TexSyntaxKind::Dollar) | Some(TexSyntaxKind::DoubleDollar) => self.parse_math(state),
            Some(TexSyntaxKind::Caret) => self.parse_superscript(state),
            Some(TexSyntaxKind::Underscore) => self.parse_subscript(state),
            _ => {
                state.bump();
                Ok(())
            }
        }
    }

    fn is_command_like(&self, kind: TexSyntaxKind) -> bool {
        match kind {
            TexSyntaxKind::Backslash |
            TexSyntaxKind::Command |
            // TexSyntaxKind::BeginKeyword | // Handled separately
            TexSyntaxKind::EndKeyword |
            TexSyntaxKind::DocumentclassKeyword |
            TexSyntaxKind::UsepackageKeyword |
            TexSyntaxKind::SectionKeyword |
            TexSyntaxKind::SubsectionKeyword |
            TexSyntaxKind::SubsubsectionKeyword |
            TexSyntaxKind::ChapterKeyword |
            TexSyntaxKind::PartKeyword |
            TexSyntaxKind::TitleKeyword |
            TexSyntaxKind::AuthorKeyword |
            TexSyntaxKind::DateKeyword |
            TexSyntaxKind::MaketitleKeyword |
            TexSyntaxKind::TableofcontentsKeyword |
            TexSyntaxKind::ItemKeyword |
            TexSyntaxKind::LabelKeyword |
            TexSyntaxKind::RefKeyword |
            TexSyntaxKind::CiteKeyword |
            TexSyntaxKind::IncludegraphicsKeyword |
            TexSyntaxKind::TextbfKeyword |
            TexSyntaxKind::TextitKeyword |
            TexSyntaxKind::EmphKeyword |
            TexSyntaxKind::Frac |
            TexSyntaxKind::Sqrt |
            TexSyntaxKind::Sum |
            TexSyntaxKind::Int |
            TexSyntaxKind::Lim |
            TexSyntaxKind::Alpha |
            TexSyntaxKind::Beta |
            TexSyntaxKind::Gamma |
            TexSyntaxKind::Delta |
            TexSyntaxKind::Epsilon => true,
            _ => false,
        }
    }

    fn parse_environment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Parse \begin{name}
        let begin_checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::BeginKeyword)?;
        if state.at(TexSyntaxKind::LeftBrace) {
            self.parse_mandatory_argument(state)?;
        }
        state.finish_at(begin_checkpoint, TexSyntaxKind::BeginEnvironment);

        // Parse content until \end{name}
        while state.not_at_end() && !state.at(TexSyntaxKind::EndKeyword) {
            self.parse_item(state)?;
        }

        // Parse \end{name}
        if state.at(TexSyntaxKind::EndKeyword) {
            let end_checkpoint = state.checkpoint();
            state.bump();
            if state.at(TexSyntaxKind::LeftBrace) {
                self.parse_mandatory_argument(state)?;
            }
            state.finish_at(end_checkpoint, TexSyntaxKind::EndEnvironment);
        }

        state.finish_at(checkpoint, TexSyntaxKind::Environment);
        Ok(())
    }

    fn parse_superscript<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::Caret)?;

        if state.at(TexSyntaxKind::LeftBrace) {
            self.parse_group(state)?;
        }
        else {
            state.bump();
        }

        state.finish_at(checkpoint, TexSyntaxKind::Superscript);
        Ok(())
    }

    fn parse_subscript<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::Underscore)?;

        if state.at(TexSyntaxKind::LeftBrace) {
            self.parse_group(state)?;
        }
        else {
            state.bump();
        }

        state.finish_at(checkpoint, TexSyntaxKind::Subscript);
        Ok(())
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().unwrap_or(TexSyntaxKind::Command);

        let should_parse_args = state.peek_text().map_or(true, |name| {
            let name_str: &str = &name;
            let name_str = name_str.strip_prefix('\\').unwrap_or(name_str);
            name_str != "left" && name_str != "right"
        });

        state.bump(); // consume \ or command keyword

        if should_parse_args {
            while state.at(TexSyntaxKind::LeftBracket) || state.at(TexSyntaxKind::LeftBrace) {
                if state.at(TexSyntaxKind::LeftBracket) {
                    self.parse_optional_argument(state)?;
                }
                else {
                    self.parse_mandatory_argument(state)?;
                }
            }
        }

        // 如果是特殊的命令种类（如 Frac, Sqrt），则保留该种类，否则使用通用的 Command
        let node_kind = match kind {
            TexSyntaxKind::Frac
            | TexSyntaxKind::Sqrt
            | TexSyntaxKind::Sum
            | TexSyntaxKind::Int
            | TexSyntaxKind::Lim
            | TexSyntaxKind::BeginKeyword
            | TexSyntaxKind::EndKeyword
            | TexSyntaxKind::SectionKeyword
            | TexSyntaxKind::SubsectionKeyword
            | TexSyntaxKind::SubsubsectionKeyword
            | TexSyntaxKind::ChapterKeyword
            | TexSyntaxKind::PartKeyword
            | TexSyntaxKind::TitleKeyword
            | TexSyntaxKind::AuthorKeyword
            | TexSyntaxKind::DateKeyword
            | TexSyntaxKind::MaketitleKeyword
            | TexSyntaxKind::TableofcontentsKeyword
            | TexSyntaxKind::ItemKeyword
            | TexSyntaxKind::LabelKeyword
            | TexSyntaxKind::RefKeyword
            | TexSyntaxKind::CiteKeyword
            | TexSyntaxKind::IncludegraphicsKeyword
            | TexSyntaxKind::TextbfKeyword
            | TexSyntaxKind::TextitKeyword
            | TexSyntaxKind::EmphKeyword
            | TexSyntaxKind::Alpha
            | TexSyntaxKind::Beta
            | TexSyntaxKind::Gamma
            | TexSyntaxKind::Delta
            | TexSyntaxKind::Epsilon
            | TexSyntaxKind::Zeta
            | TexSyntaxKind::Eta
            | TexSyntaxKind::Theta
            | TexSyntaxKind::Iota
            | TexSyntaxKind::Kappa
            | TexSyntaxKind::Lambda
            | TexSyntaxKind::Mu
            | TexSyntaxKind::Nu
            | TexSyntaxKind::Xi
            | TexSyntaxKind::Omicron
            | TexSyntaxKind::Pi
            | TexSyntaxKind::Rho
            | TexSyntaxKind::Sigma
            | TexSyntaxKind::Tau
            | TexSyntaxKind::Upsilon
            | TexSyntaxKind::Phi
            | TexSyntaxKind::Chi
            | TexSyntaxKind::Psi
            | TexSyntaxKind::Omega
            | TexSyntaxKind::VarEpsilon
            | TexSyntaxKind::VarTheta
            | TexSyntaxKind::VarKappa
            | TexSyntaxKind::VarPi
            | TexSyntaxKind::VarRho
            | TexSyntaxKind::VarSigma
            | TexSyntaxKind::VarPhi
            | TexSyntaxKind::UpperGamma
            | TexSyntaxKind::UpperDelta
            | TexSyntaxKind::UpperTheta
            | TexSyntaxKind::UpperLambda
            | TexSyntaxKind::UpperXi
            | TexSyntaxKind::UpperPi
            | TexSyntaxKind::UpperSigma
            | TexSyntaxKind::UpperUpsilon
            | TexSyntaxKind::UpperPhi
            | TexSyntaxKind::UpperPsi
            | TexSyntaxKind::UpperOmega => kind,
            _ => TexSyntaxKind::Command,
        };

        state.finish_at(checkpoint, node_kind);
        Ok(())
    }

    fn parse_group<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::LeftBrace)?;
        while state.not_at_end() && !state.at(TexSyntaxKind::RightBrace) {
            self.parse_item(state)?;
        }
        state.expect(TexSyntaxKind::RightBrace)?;
        state.finish_at(checkpoint, TexSyntaxKind::Group);
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
        let node_kind = if kind == TexSyntaxKind::DoubleDollar { TexSyntaxKind::DisplayMath } else { TexSyntaxKind::InlineMath };
        state.finish_at(checkpoint, node_kind);
        Ok(())
    }

    fn parse_optional_argument<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::LeftBracket)?;
        while state.not_at_end() && !state.at(TexSyntaxKind::RightBracket) {
            self.parse_item(state)?;
        }
        state.expect(TexSyntaxKind::RightBracket)?;
        state.finish_at(checkpoint, TexSyntaxKind::OptionalArgument);
        Ok(())
    }

    fn parse_mandatory_argument<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TexSyntaxKind::LeftBrace)?;
        while state.not_at_end() && !state.at(TexSyntaxKind::RightBrace) {
            self.parse_item(state)?;
        }
        state.expect(TexSyntaxKind::RightBrace)?;
        state.finish_at(checkpoint, TexSyntaxKind::MandatoryArgument);
        Ok(())
    }
}
