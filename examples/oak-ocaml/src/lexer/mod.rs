#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::OCamlLanguage, lexer::token_type::OCamlTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, OCamlLanguage>;

static OCAML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static OCAML_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "(*", block_end: "*)", nested_blocks: true });

#[derive(Clone, Debug)]
pub struct OCamlLexer<'config> {
    _config: &'config OCamlLanguage,
}

impl<'config> Lexer<OCamlLanguage> for OCamlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<OCamlLanguage>) -> LexOutput<OCamlLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> OCamlLexer<'config> {
    pub fn new(config: &'config OCamlLanguage) -> Self {
        Self { _config: config }
    }

    /// 主词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        OCAML_WHITESPACE.scan(state, OCamlTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        OCAML_COMMENT.scan(state, OCamlTokenType::Comment, OCamlTokenType::Comment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
        state.add_token(OCamlTokenType::StringLiteral, start, state.get_position());
        true
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('\'') {
            return false;
        }

        state.advance(1); // opening '
        if let Some('\\') = state.peek() {
            state.advance(1);
            if let Some(c) = state.peek() {
                state.advance(c.len_utf8())
            }
        }
        else if let Some(c) = state.peek() {
            state.advance(c.len_utf8())
        }
        else {
            state.set_position(start);
            return false;
        }

        if state.peek() == Some('\'') {
            state.advance(1);
            state.add_token(OCamlTokenType::CharLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
        state.add_token(if is_float { OCamlTokenType::FloatLiteral } else { OCamlTokenType::IntegerLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' { state.advance(1) } else { break }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text.as_ref() {
            // OCaml keywords
            "and" => OCamlTokenType::And,
            "as" => OCamlTokenType::As,
            "assert" => OCamlTokenType::Assert,
            "begin" => OCamlTokenType::Begin,
            "class" => OCamlTokenType::Class,
            "constraint" => OCamlTokenType::Constraint,
            "do" => OCamlTokenType::Do,
            "done" => OCamlTokenType::Done,
            "downto" => OCamlTokenType::Downto,
            "else" => OCamlTokenType::Else,
            "end" => OCamlTokenType::End,
            "exception" => OCamlTokenType::Exception,
            "external" => OCamlTokenType::External,
            "false" => OCamlTokenType::False,
            "for" => OCamlTokenType::For,
            "fun" => OCamlTokenType::Fun,
            "function" => OCamlTokenType::Function,
            "functor" => OCamlTokenType::Functor,
            "if" => OCamlTokenType::If,
            "in" => OCamlTokenType::In,
            "include" => OCamlTokenType::Include,
            "inherit" => OCamlTokenType::Inherit,
            "initializer" => OCamlTokenType::Initializer,
            "lazy" => OCamlTokenType::Lazy,
            "let" => OCamlTokenType::Let,
            "match" => OCamlTokenType::Match,
            "method" => OCamlTokenType::Method,
            "module" => OCamlTokenType::Module,
            "mutable" => OCamlTokenType::Mutable,
            "new" => OCamlTokenType::New,
            "object" => OCamlTokenType::Object,
            "of" => OCamlTokenType::Of,
            "open" => OCamlTokenType::Open,
            "or" => OCamlTokenType::Or,
            "private" => OCamlTokenType::Private,
            "rec" => OCamlTokenType::Rec,
            "sig" => OCamlTokenType::Sig,
            "struct" => OCamlTokenType::Struct,
            "then" => OCamlTokenType::Then,
            "to" => OCamlTokenType::To,
            "true" => OCamlTokenType::True,
            "try" => OCamlTokenType::Try,
            "type" => OCamlTokenType::Type,
            "val" => OCamlTokenType::Val,
            "virtual" => OCamlTokenType::Virtual,
            "when" => OCamlTokenType::When,
            "while" => OCamlTokenType::While,
            "with" => OCamlTokenType::With,

            _ => OCamlTokenType::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // prefer longest matches first
        let patterns: &[(&str, OCamlTokenType)] = &[
            ("==", OCamlTokenType::EqualEqual),
            ("!=", OCamlTokenType::NotEqual),
            (">=", OCamlTokenType::GreaterEqual),
            ("<=", OCamlTokenType::LessEqual),
            ("&&", OCamlTokenType::AndAnd),
            ("||", OCamlTokenType::OrOr),
            ("::", OCamlTokenType::ColonColon),
            ("->", OCamlTokenType::RightArrow),
            ("<-", OCamlTokenType::LeftArrow),
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
                '+' => Some(OCamlTokenType::Plus),
                '-' => Some(OCamlTokenType::Minus),
                '*' => Some(OCamlTokenType::Star),
                '/' => Some(OCamlTokenType::Slash),
                '%' => Some(OCamlTokenType::Percent),
                '=' => Some(OCamlTokenType::Equal),
                '>' => Some(OCamlTokenType::Greater),
                '<' => Some(OCamlTokenType::Less),
                '!' => Some(OCamlTokenType::Bang),
                '?' => Some(OCamlTokenType::Question),
                ':' => Some(OCamlTokenType::Colon),
                ';' => Some(OCamlTokenType::Semicolon),
                ',' => Some(OCamlTokenType::Comma),
                '.' => Some(OCamlTokenType::Dot),
                '|' => Some(OCamlTokenType::Pipe),
                '&' => Some(OCamlTokenType::Ampersand),
                '^' => Some(OCamlTokenType::Caret),
                '~' => Some(OCamlTokenType::Tilde),
                '@' => Some(OCamlTokenType::At),
                '#' => Some(OCamlTokenType::Hash),
                '$' => Some(OCamlTokenType::Dollar),
                '`' => Some(OCamlTokenType::Backtick),
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

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => OCamlTokenType::LeftParen,
                ')' => OCamlTokenType::RightParen,
                '[' => OCamlTokenType::LeftBracket,
                ']' => OCamlTokenType::RightBracket,
                '{' => OCamlTokenType::LeftBrace,
                '}' => OCamlTokenType::RightBrace,
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
