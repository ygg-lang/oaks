use crate::{
    kind::{VampireSyntaxKind, VampireToken},
    language::VampireLanguage,
};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, VampireLanguage>;

static VAMPIRE_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VAMPIRE_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["%"] });
static VAMPIRE_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct VampireLexer<'config> {
    config: &'config VampireLanguage,
}

impl<'config> Lexer<VampireLanguage> for VampireLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<VampireLanguage>,
    ) -> LexOutput<VampireLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> VampireLexer<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }

    /// Main lexing loop
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // Add EOF token
        let eof_pos = state.get_position();
        state.add_token(VampireSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// Skip whitespace characters
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match VAMPIRE_WHITESPACE.scan(state.rest(), state.get_position(), VampireSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// Skip comment lines
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Line comment: % ... until newline
        if rest.starts_with("%") {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VampireSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // Block comment: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VampireSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    /// Lex string literals
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('"') {
            state.advance(1);
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if ch == '"' && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(VampireSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    /// Lex number literals
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() && first != '-' && first != '+' {
            return false;
        }

        let mut is_real = false;

        // Handle sign
        if first == '-' || first == '+' {
            state.advance(1);
            if !state.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                state.set_position(start);
                return false;
            }
        }

        // Integer part
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // Fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_real = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // Exponent
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_real = true;
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        let end = state.get_position();
        state.add_token(if is_real { VampireSyntaxKind::RealLiteral } else { VampireSyntaxKind::IntegerLiteral }, start, end);
        true
    }

    /// Lex identifiers and keywords
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '$') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
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
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    /// Lex operators
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Multi-character operators (longest first)
        let patterns: &[(&str, VampireSyntaxKind)] = &[
            ("<==>", VampireSyntaxKind::IffKw),
            ("<~>", VampireSyntaxKind::XorKw),
            ("==>", VampireSyntaxKind::ImpliesKw),
            ("~|", VampireSyntaxKind::NorKw),
            ("~&", VampireSyntaxKind::NandKw),
            ("==", VampireSyntaxKind::EqEq),
            ("!=", VampireSyntaxKind::NotEq),
            ("<=", VampireSyntaxKind::LessEq),
            (">=", VampireSyntaxKind::GreaterEq),
            ("&&", VampireSyntaxKind::AndAnd),
            ("||", VampireSyntaxKind::OrOr),
            ("++", VampireSyntaxKind::PlusPlus),
            ("--", VampireSyntaxKind::MinusMinus),
            ("+=", VampireSyntaxKind::PlusEq),
            ("-=", VampireSyntaxKind::MinusEq),
            ("*=", VampireSyntaxKind::StarEq),
            ("/=", VampireSyntaxKind::SlashEq),
            ("%=", VampireSyntaxKind::PercentEq),
            ("<<", VampireSyntaxKind::LeftShift),
            (">>", VampireSyntaxKind::RightShift),
            ("->", VampireSyntaxKind::Arrow),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // Single character operators
        if let Some(ch) = state.current() {
            let kind = match ch {
                '=' => Some(VampireSyntaxKind::Eq),
                '<' => Some(VampireSyntaxKind::LessThan),
                '>' => Some(VampireSyntaxKind::GreaterThan),
                '+' => Some(VampireSyntaxKind::Plus),
                '-' => Some(VampireSyntaxKind::Minus),
                '*' => Some(VampireSyntaxKind::Star),
                '/' => Some(VampireSyntaxKind::Slash),
                '\\' => Some(VampireSyntaxKind::Backslash),
                '%' => Some(VampireSyntaxKind::Percent),
                '^' => Some(VampireSyntaxKind::Caret),
                '&' => Some(VampireSyntaxKind::Ampersand),
                '|' => Some(VampireSyntaxKind::Pipe),
                '~' => Some(VampireSyntaxKind::Tilde),
                '!' => Some(VampireSyntaxKind::Bang),
                '?' => Some(VampireSyntaxKind::Question),
                '.' => Some(VampireSyntaxKind::Dot),
                ':' => Some(VampireSyntaxKind::Colon),
                _ => None,
            };

            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Lex single character tokens
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => VampireSyntaxKind::LeftParen,
                ')' => VampireSyntaxKind::RightParen,
                '[' => VampireSyntaxKind::LeftBracket,
                ']' => VampireSyntaxKind::RightBracket,
                '{' => VampireSyntaxKind::LeftBrace,
                '}' => VampireSyntaxKind::RightBrace,
                ',' => VampireSyntaxKind::Comma,
                ';' => VampireSyntaxKind::Semicolon,
                '@' => VampireSyntaxKind::At,
                '#' => VampireSyntaxKind::Hash,
                '$' => VampireSyntaxKind::Dollar,
                _ => {
                    // Unknown character, create error token
                    state.advance(ch.len_utf8());
                    state.add_token(VampireSyntaxKind::Error, start, state.get_position());
                    return true;
                }
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
