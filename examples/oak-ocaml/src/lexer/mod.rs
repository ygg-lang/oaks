use crate::{OCamlToken, kind::OCamlSyntaxKind, language::OCamlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, OCamlLanguage>;

static OCAML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static OCAML_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static OCAML_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static OCAML_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct OCamlLexer<'config> {
    config: &'config OCamlLanguage,
}

impl<'config> Lexer<OCamlLanguage> for OCamlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<OCamlLanguage>,
    ) -> LexOutput<OCamlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> OCamlLexer<'config> {
    pub fn new(config: &'config OCamlLanguage) -> Self {
        Self { config }
    }

    /// 主词法分析循环
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

            if self.lex_char_literal(state) {
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

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(OCamlSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match OCAML_WHITESPACE.scan(state.rest(), state.get_position(), OCamlSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // OCaml block comment: (* ... *)
        if rest.starts_with("(*") {
            state.advance(2);
            let mut depth = 1;
            while let Some(ch) = state.peek() {
                if ch == '(' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(OCamlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('"') {
            return false;
        }

        state.advance(1); // opening "
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
        state.add_token(OCamlSyntaxKind::StringLiteral, start, state.get_position());
        true
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('\'') {
            return false;
        }

        state.advance(1); // opening '
        if let Some('\\') = state.peek() {
            state.advance(1);
            if let Some(c) = state.peek() {
                state.advance(c.len_utf8());
            }
        }
        else if let Some(c) = state.peek() {
            state.advance(c.len_utf8());
        }
        else {
            state.set_position(start);
            return false;
        }

        if state.peek() == Some('\'') {
            state.advance(1);
            state.add_token(OCamlSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;

        // consume digits
        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
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

        // exponent
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_float = true;
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
        state.add_token(if is_float { OCamlSyntaxKind::FloatLiteral } else { OCamlSyntaxKind::IntegerLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            // OCaml keywords
            "and" => OCamlSyntaxKind::And,
            "as" => OCamlSyntaxKind::As,
            "assert" => OCamlSyntaxKind::Assert,
            "begin" => OCamlSyntaxKind::Begin,
            "class" => OCamlSyntaxKind::Class,
            "constraint" => OCamlSyntaxKind::Constraint,
            "do" => OCamlSyntaxKind::Do,
            "done" => OCamlSyntaxKind::Done,
            "downto" => OCamlSyntaxKind::Downto,
            "else" => OCamlSyntaxKind::Else,
            "end" => OCamlSyntaxKind::End,
            "exception" => OCamlSyntaxKind::Exception,
            "external" => OCamlSyntaxKind::External,
            "false" => OCamlSyntaxKind::False,
            "for" => OCamlSyntaxKind::For,
            "fun" => OCamlSyntaxKind::Fun,
            "function" => OCamlSyntaxKind::Function,
            "functor" => OCamlSyntaxKind::Functor,
            "if" => OCamlSyntaxKind::If,
            "in" => OCamlSyntaxKind::In,
            "include" => OCamlSyntaxKind::Include,
            "inherit" => OCamlSyntaxKind::Inherit,
            "initializer" => OCamlSyntaxKind::Initializer,
            "lazy" => OCamlSyntaxKind::Lazy,
            "let" => OCamlSyntaxKind::Let,
            "match" => OCamlSyntaxKind::Match,
            "method" => OCamlSyntaxKind::Method,
            "module" => OCamlSyntaxKind::Module,
            "mutable" => OCamlSyntaxKind::Mutable,
            "new" => OCamlSyntaxKind::New,
            "object" => OCamlSyntaxKind::Object,
            "of" => OCamlSyntaxKind::Of,
            "open" => OCamlSyntaxKind::Open,
            "or" => OCamlSyntaxKind::Or,
            "private" => OCamlSyntaxKind::Private,
            "rec" => OCamlSyntaxKind::Rec,
            "sig" => OCamlSyntaxKind::Sig,
            "struct" => OCamlSyntaxKind::Struct,
            "then" => OCamlSyntaxKind::Then,
            "to" => OCamlSyntaxKind::To,
            "true" => OCamlSyntaxKind::True,
            "try" => OCamlSyntaxKind::Try,
            "type" => OCamlSyntaxKind::Type,
            "val" => OCamlSyntaxKind::Val,
            "virtual" => OCamlSyntaxKind::Virtual,
            "when" => OCamlSyntaxKind::When,
            "while" => OCamlSyntaxKind::While,
            "with" => OCamlSyntaxKind::With,

            _ => OCamlSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // prefer longest matches first
        let patterns: &[(&str, OCamlSyntaxKind)] = &[
            ("==", OCamlSyntaxKind::EqualEqual),
            ("!=", OCamlSyntaxKind::NotEqual),
            (">=", OCamlSyntaxKind::GreaterEqual),
            ("<=", OCamlSyntaxKind::LessEqual),
            ("&&", OCamlSyntaxKind::AndAnd),
            ("||", OCamlSyntaxKind::OrOr),
            ("::", OCamlSyntaxKind::ColonColon),
            ("->", OCamlSyntaxKind::RightArrow),
            ("<-", OCamlSyntaxKind::LeftArrow),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(OCamlSyntaxKind::Plus),
                '-' => Some(OCamlSyntaxKind::Minus),
                '*' => Some(OCamlSyntaxKind::Star),
                '/' => Some(OCamlSyntaxKind::Slash),
                '%' => Some(OCamlSyntaxKind::Percent),
                '=' => Some(OCamlSyntaxKind::Equal),
                '>' => Some(OCamlSyntaxKind::Greater),
                '<' => Some(OCamlSyntaxKind::Less),
                '!' => Some(OCamlSyntaxKind::Bang),
                '?' => Some(OCamlSyntaxKind::Question),
                ':' => Some(OCamlSyntaxKind::Colon),
                ';' => Some(OCamlSyntaxKind::Semicolon),
                ',' => Some(OCamlSyntaxKind::Comma),
                '.' => Some(OCamlSyntaxKind::Dot),
                '|' => Some(OCamlSyntaxKind::Pipe),
                '&' => Some(OCamlSyntaxKind::Ampersand),
                '^' => Some(OCamlSyntaxKind::Caret),
                '~' => Some(OCamlSyntaxKind::Tilde),
                '@' => Some(OCamlSyntaxKind::At),
                '#' => Some(OCamlSyntaxKind::Hash),
                '$' => Some(OCamlSyntaxKind::Dollar),
                '`' => Some(OCamlSyntaxKind::Backtick),
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

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => OCamlSyntaxKind::LeftParen,
                ')' => OCamlSyntaxKind::RightParen,
                '[' => OCamlSyntaxKind::LeftBracket,
                ']' => OCamlSyntaxKind::RightBracket,
                '{' => OCamlSyntaxKind::LeftBrace,
                '}' => OCamlSyntaxKind::RightBrace,
                _ => return false,
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
