use crate::{kind::VampireSyntaxKind, language::VampireLanguage};
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
        VAMPIRE_WHITESPACE.scan(state, VampireSyntaxKind::Whitespace)
    }

    /// Skip comment lines
    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        VAMPIRE_COMMENT.scan(state, VampireSyntaxKind::LineComment, VampireSyntaxKind::BlockComment)
    }

    /// Lex string literals
    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        VAMPIRE_STRING.scan(state, VampireSyntaxKind::StringLiteral)
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

        let kind = if is_float { VampireSyntaxKind::RealLiteral } else { VampireSyntaxKind::IntegerLiteral };
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
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
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
    fn keyword_or_identifier(&self, text: &str) -> VampireSyntaxKind {
        match text {
            // TPTP formula types
            "fof" => VampireSyntaxKind::FofKw,
            "cnf" => VampireSyntaxKind::CnfKw,
            "tff" => VampireSyntaxKind::TffKw,
            "thf" => VampireSyntaxKind::ThfKw,
            "tpi" => VampireSyntaxKind::TpiKw,
            "include" => VampireSyntaxKind::IncludeKw,

            // Formula roles
            "axiom" => VampireSyntaxKind::AxiomKw,
            "hypothesis" => VampireSyntaxKind::HypothesisKw,
            "definition" => VampireSyntaxKind::DefinitionKw,
            "assumption" => VampireSyntaxKind::AssumptionKw,
            "lemma" => VampireSyntaxKind::LemmaKw,
            "theorem" => VampireSyntaxKind::TheoremKw,
            "conjecture" => VampireSyntaxKind::ConjectureKw,
            "negated_conjecture" => VampireSyntaxKind::NegatedConjectureKw,
            "plain" => VampireSyntaxKind::PlainKw,
            "type" => VampireSyntaxKind::TypeKw,
            "fi_domain" => VampireSyntaxKind::FiDomainKw,
            "fi_functors" => VampireSyntaxKind::FiFunctorsKw,
            "fi_predicates" => VampireSyntaxKind::FiPredicatesKw,
            "unknown" => VampireSyntaxKind::UnknownKw,

            // Logical operators
            "!" => VampireSyntaxKind::ForallKw,
            "?" => VampireSyntaxKind::ExistsKw,
            "&" => VampireSyntaxKind::AndKw,
            "|" => VampireSyntaxKind::OrKw,
            "~" => VampireSyntaxKind::NotKw,
            "=>" => VampireSyntaxKind::ImpliesKw,
            "<=>" => VampireSyntaxKind::IffKw,
            "<~>" => VampireSyntaxKind::XorKw,
            "~|" => VampireSyntaxKind::NorKw,
            "~&" => VampireSyntaxKind::NandKw,

            // Types
            "$o" => VampireSyntaxKind::BoolKw,
            "$i" => VampireSyntaxKind::IndividualKw,
            "$int" => VampireSyntaxKind::IntKw,
            "$real" => VampireSyntaxKind::RealKw,
            "$rat" => VampireSyntaxKind::RatKw,
            "$tType" => VampireSyntaxKind::TTypeKw,
            "$oType" => VampireSyntaxKind::OTypeKw,
            "$iType" => VampireSyntaxKind::ITypeKw,

            // Boolean literals
            "$true" => VampireSyntaxKind::BoolLiteral,
            "$false" => VampireSyntaxKind::BoolLiteral,

            _ => VampireSyntaxKind::Identifier,
        }
    }

    /// Lex operators
    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        let (kind, len) = if rest.starts_with("==>") {
            (VampireSyntaxKind::ImpliesKw, 3)
        }
        else if rest.starts_with("<=>") {
            (VampireSyntaxKind::IffKw, 3)
        }
        else if rest.starts_with("<~>") {
            (VampireSyntaxKind::XorKw, 3)
        }
        else if rest.starts_with("~|") {
            (VampireSyntaxKind::NorKw, 2)
        }
        else if rest.starts_with("~&") {
            (VampireSyntaxKind::NandKw, 2)
        }
        else if rest.starts_with("==") {
            (VampireSyntaxKind::EqEq, 2)
        }
        else if rest.starts_with("!=") {
            (VampireSyntaxKind::NotEq, 2)
        }
        else if rest.starts_with("<=") {
            (VampireSyntaxKind::LessEq, 2)
        }
        else if rest.starts_with(">=") {
            (VampireSyntaxKind::GreaterEq, 2)
        }
        else if rest.starts_with("&&") {
            (VampireSyntaxKind::AndAnd, 2)
        }
        else if rest.starts_with("||") {
            (VampireSyntaxKind::OrOr, 2)
        }
        else if rest.starts_with("++") {
            (VampireSyntaxKind::PlusPlus, 2)
        }
        else if rest.starts_with("--") {
            (VampireSyntaxKind::MinusMinus, 2)
        }
        else if rest.starts_with("+=") {
            (VampireSyntaxKind::PlusEq, 2)
        }
        else if rest.starts_with("-=") {
            (VampireSyntaxKind::MinusEq, 2)
        }
        else if rest.starts_with("*=") {
            (VampireSyntaxKind::StarEq, 2)
        }
        else if rest.starts_with("/=") {
            (VampireSyntaxKind::SlashEq, 2)
        }
        else if rest.starts_with("%=") {
            (VampireSyntaxKind::PercentEq, 2)
        }
        else if rest.starts_with("<<") {
            (VampireSyntaxKind::LeftShift, 2)
        }
        else if rest.starts_with(">>") {
            (VampireSyntaxKind::RightShift, 2)
        }
        else if rest.starts_with("->") {
            (VampireSyntaxKind::Arrow, 2)
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
                '(' => Some(VampireSyntaxKind::LeftParen),
                ')' => Some(VampireSyntaxKind::RightParen),
                '[' => Some(VampireSyntaxKind::LeftBracket),
                ']' => Some(VampireSyntaxKind::RightBracket),
                '{' => Some(VampireSyntaxKind::LeftBrace),
                '}' => Some(VampireSyntaxKind::RightBrace),
                ':' => Some(VampireSyntaxKind::Colon),
                ';' => Some(VampireSyntaxKind::Semicolon),
                '.' => Some(VampireSyntaxKind::Dot),
                ',' => Some(VampireSyntaxKind::Comma),
                '?' => Some(VampireSyntaxKind::Question),
                '!' => Some(VampireSyntaxKind::Bang),
                '@' => Some(VampireSyntaxKind::At),
                '#' => Some(VampireSyntaxKind::Hash),
                '$' => Some(VampireSyntaxKind::Dollar),
                '%' => Some(VampireSyntaxKind::Percent),
                '^' => Some(VampireSyntaxKind::Caret),
                '&' => Some(VampireSyntaxKind::Ampersand),
                '*' => Some(VampireSyntaxKind::Star),
                '+' => Some(VampireSyntaxKind::Plus),
                '-' => Some(VampireSyntaxKind::Minus),
                '=' => Some(VampireSyntaxKind::Eq),
                '<' => Some(VampireSyntaxKind::LessThan),
                '>' => Some(VampireSyntaxKind::GreaterThan),
                '/' => Some(VampireSyntaxKind::Slash),
                '\\' => Some(VampireSyntaxKind::Backslash),
                '|' => Some(VampireSyntaxKind::Pipe),
                '~' => Some(VampireSyntaxKind::Tilde),
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
