#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::VampireLanguage, lexer::token_type::VampireTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, VampireLanguage>;

static VAMPIRE_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VAMPIRE_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "%", block_start: "/*", block_end: "*/", nested_blocks: false });
static VAMPIRE_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct VampireLexer<'config> {
    _config: &'config VampireLanguage,
}

impl<'config> Lexer<VampireLanguage> for VampireLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], _cache: &'a mut impl LexerCache<VampireLanguage>) -> LexOutput<VampireLanguage> {
        let mut state: State<'_, S> = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish(result)
    }
}
impl<'config> VampireLexer<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { _config: config }
    }

    /// Main lexing loop
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// Skip whitespace characters
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        VAMPIRE_WHITESPACE.scan(state, VampireTokenType::Whitespace)
    }

    /// Skip comment lines
    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        VAMPIRE_COMMENT.scan(state, VampireTokenType::LineComment, VampireTokenType::BlockComment)
    }

    /// Lex string literals
    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        VAMPIRE_STRING.scan(state, VampireTokenType::StringLiteral)
    }

    /// Lex number literals
    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let mut has_digit = false;

        // Optional minus
        if state.peek() == Some('-') {
            state.advance(1);
        }

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
                has_digit = true;
            }
            else {
                break;
            }
        }

        if !has_digit {
            return false;
        }

        // Check for float (dot or exponent)
        let mut is_float = false;
        if state.peek() == Some('.') {
            if let Some(next) = state.peek_next_n(1) {
                if next.is_ascii_digit() {
                    state.advance(1); // skip dot
                    is_float = true;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        if let Some(ch) = state.peek() {
            if ch == 'e' || ch == 'E' {
                state.advance(1);
                is_float = true;
                if let Some(ch) = state.peek() {
                    if ch == '+' || ch == '-' {
                        state.advance(1);
                    }
                }
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
        }

        let kind = if is_float { VampireTokenType::RealLiteral } else { VampireTokenType::IntegerLiteral };
        state.add_token(kind, start, state.get_position());
        true
    }

    /// Lex identifier or keyword
    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' { state.advance(ch.len_utf8()) } else { break }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = self.keyword_or_identifier(text.as_ref());
                state.add_token(kind, start, end);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Determine if text is a keyword or identifier
    fn keyword_or_identifier(&self, text: &str) -> VampireTokenType {
        match text {
            // TPTP formula types
            "fof" => VampireTokenType::FofKw,
            "cnf" => VampireTokenType::CnfKw,
            "tff" => VampireTokenType::TffKw,
            "thf" => VampireTokenType::ThfKw,
            "tpi" => VampireTokenType::TpiKw,
            "include" => VampireTokenType::IncludeKw,

            // Formula roles
            "axiom" => VampireTokenType::AxiomKw,
            "hypothesis" => VampireTokenType::HypothesisKw,
            "definition" => VampireTokenType::DefinitionKw,
            "assumption" => VampireTokenType::AssumptionKw,
            "lemma" => VampireTokenType::LemmaKw,
            "theorem" => VampireTokenType::TheoremKw,
            "conjecture" => VampireTokenType::ConjectureKw,
            "negated_conjecture" => VampireTokenType::NegatedConjectureKw,
            "plain" => VampireTokenType::PlainKw,
            "type" => VampireTokenType::TypeKw,
            "fi_domain" => VampireTokenType::FiDomainKw,
            "fi_functors" => VampireTokenType::FiFunctorsKw,
            "fi_predicates" => VampireTokenType::FiPredicatesKw,
            "unknown" => VampireTokenType::UnknownKw,

            // Logical operators
            "!" => VampireTokenType::ForallKw,
            "?" => VampireTokenType::ExistsKw,
            "&" => VampireTokenType::AndKw,
            "|" => VampireTokenType::OrKw,
            "~" => VampireTokenType::NotKw,
            "=>" => VampireTokenType::ImpliesKw,
            "<=>" => VampireTokenType::IffKw,
            "<~>" => VampireTokenType::XorKw,
            "~|" => VampireTokenType::NorKw,
            "~&" => VampireTokenType::NandKw,

            // Types
            "$o" => VampireTokenType::BoolKw,
            "$i" => VampireTokenType::IndividualKw,
            "$int" => VampireTokenType::IntKw,
            "$real" => VampireTokenType::RealKw,
            "$rat" => VampireTokenType::RatKw,
            "$tType" => VampireTokenType::TTypeKw,
            "$oType" => VampireTokenType::OTypeKw,
            "$iType" => VampireTokenType::ITypeKw,

            // Boolean literals
            "$true" => VampireTokenType::BoolLiteral,
            "$false" => VampireTokenType::BoolLiteral,

            _ => VampireTokenType::Identifier,
        }
    }

    /// Lex operators
    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        let (kind, len) = if rest.starts_with("==>") {
            (VampireTokenType::ImpliesKw, 3)
        }
        else if rest.starts_with("<=>") {
            (VampireTokenType::IffKw, 3)
        }
        else if rest.starts_with("<~>") {
            (VampireTokenType::XorKw, 3)
        }
        else if rest.starts_with("~|") {
            (VampireTokenType::NorKw, 2)
        }
        else if rest.starts_with("~&") {
            (VampireTokenType::NandKw, 2)
        }
        else if rest.starts_with("==") {
            (VampireTokenType::DoubleEq, 2)
        }
        else if rest.starts_with("!=") {
            (VampireTokenType::NotEq, 2)
        }
        else if rest.starts_with("<=") {
            (VampireTokenType::LessEq, 2)
        }
        else if rest.starts_with(">=") {
            (VampireTokenType::GreaterEq, 2)
        }
        else if rest.starts_with("&&") {
            (VampireTokenType::AndAnd, 2)
        }
        else if rest.starts_with("||") {
            (VampireTokenType::OrOr, 2)
        }
        else if rest.starts_with("++") {
            (VampireTokenType::PlusPlus, 2)
        }
        else if rest.starts_with("--") {
            (VampireTokenType::MinusMinus, 2)
        }
        else if rest.starts_with("+=") {
            (VampireTokenType::PlusEq, 2)
        }
        else if rest.starts_with("-=") {
            (VampireTokenType::MinusEq, 2)
        }
        else if rest.starts_with("*=") {
            (VampireTokenType::StarEq, 2)
        }
        else if rest.starts_with("/=") {
            (VampireTokenType::SlashEq, 2)
        }
        else if rest.starts_with("%=") {
            (VampireTokenType::PercentEq, 2)
        }
        else if rest.starts_with("<<") {
            (VampireTokenType::LeftShift, 2)
        }
        else if rest.starts_with(">>") {
            (VampireTokenType::RightShift, 2)
        }
        else if rest.starts_with("->") {
            (VampireTokenType::Arrow, 2)
        }
        else {
            return false;
        };

        state.advance(len);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// Lex single character tokens
    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '(' => Some(VampireTokenType::LeftParen),
                ')' => Some(VampireTokenType::RightParen),
                '[' => Some(VampireTokenType::LeftBracket),
                ']' => Some(VampireTokenType::RightBracket),
                '{' => Some(VampireTokenType::LeftBrace),
                '}' => Some(VampireTokenType::RightBrace),
                ':' => Some(VampireTokenType::Colon),
                ';' => Some(VampireTokenType::Semicolon),
                '.' => Some(VampireTokenType::Dot),
                ',' => Some(VampireTokenType::Comma),
                '?' => Some(VampireTokenType::Question),
                '!' => Some(VampireTokenType::Bang),
                '@' => Some(VampireTokenType::At),
                '#' => Some(VampireTokenType::Hash),
                '$' => Some(VampireTokenType::Dollar),
                '%' => Some(VampireTokenType::Percent),
                '^' => Some(VampireTokenType::Caret),
                '&' => Some(VampireTokenType::Ampersand),
                '*' => Some(VampireTokenType::Star),
                '+' => Some(VampireTokenType::Plus),
                '-' => Some(VampireTokenType::Minus),
                '=' => Some(VampireTokenType::Eq),
                '<' => Some(VampireTokenType::LessThan),
                '>' => Some(VampireTokenType::GreaterThan),
                '/' => Some(VampireTokenType::Slash),
                '\\' => Some(VampireTokenType::Backslash),
                '|' => Some(VampireTokenType::Pipe),
                '~' => Some(VampireTokenType::Tilde),
                _ => None,
            };

            if let Some(token_kind) = kind {
                state.advance(ch.len_utf8());
                state.add_token(token_kind, start, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
}
