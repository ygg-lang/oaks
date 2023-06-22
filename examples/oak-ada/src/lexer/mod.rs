pub mod token_type;

pub use token_type::AdaTokenType;

use crate::language::AdaLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, AdaLanguage>;

static ADA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone, Debug)]
pub struct AdaLexer<'config> {
    config: &'config AdaLanguage,
}

impl<'config> Lexer<AdaLanguage> for AdaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<AdaLanguage>) -> LexOutput<AdaLanguage> {
        let mut state: State<'_, S> = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> AdaLexer<'config> {
    pub fn new(config: &'config AdaLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
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

            // 如果没有匹配任何模式，跳过当前字符并生成 Error token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(AdaTokenType::Error, safe_point, state.get_position());
            }
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ADA_WHITESPACE.scan(state, AdaTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Ada line comment: -- ... until newline
        if state.consume_if_starts_with("--") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(AdaTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Ada string: "..."
        if state.peek() == Some('"') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // consume closing quote
                    if state.peek() == Some('"') {
                        // Double quotes in Ada strings are escaped quotes
                        state.advance(1);
                        continue;
                    }
                    break;
                }
                state.advance(ch.len_utf8());
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(AdaTokenType::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.peek() != Some('\'') {
            return false;
        }

        // try parse 'x' etc.; if fails, revert
        state.advance(1); // opening '
        if let Some(c) = state.peek() {
            state.advance(c.len_utf8());
        }
        else {
            state.set_position(start);
            return false;
        }

        if state.peek() == Some('\'') {
            state.advance(1);
            state.add_token(AdaTokenType::CharacterLiteral, start, state.get_position());
            return true;
        }
        state.set_position(start);
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // consume digits
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // check for decimal point
                if state.peek() == Some('.') {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // check for exponent
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
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

                state.add_token(AdaTokenType::NumberLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = match text.to_lowercase().as_str() {
                    "abort" => AdaTokenType::Abort,
                    "abs" => AdaTokenType::Abs,
                    "abstract" => AdaTokenType::Abstract,
                    "accept" => AdaTokenType::Accept,
                    "access" => AdaTokenType::Access,
                    "aliased" => AdaTokenType::Aliased,
                    "all" => AdaTokenType::All,
                    "and" => AdaTokenType::And,
                    "array" => AdaTokenType::Array,
                    "at" => AdaTokenType::At,
                    "begin" => AdaTokenType::Begin,
                    "body" => AdaTokenType::Body,
                    "case" => AdaTokenType::Case,
                    "constant" => AdaTokenType::Constant,
                    "declare" => AdaTokenType::Declare,
                    "delay" => AdaTokenType::Delay,
                    "delta" => AdaTokenType::Delta,
                    "digits" => AdaTokenType::Digits,
                    "do" => AdaTokenType::Do,
                    "else" => AdaTokenType::Else,
                    "elsif" => AdaTokenType::Elsif,
                    "end" => AdaTokenType::End,
                    "entry" => AdaTokenType::Entry,
                    "exception" => AdaTokenType::Exception,
                    "exit" => AdaTokenType::Exit,
                    "for" => AdaTokenType::For,
                    "function" => AdaTokenType::Function,
                    "generic" => AdaTokenType::Generic,
                    "goto" => AdaTokenType::Goto,
                    "if" => AdaTokenType::If,
                    "in" => AdaTokenType::In,
                    "interface" => AdaTokenType::Interface,
                    "is" => AdaTokenType::Is,
                    "limited" => AdaTokenType::Limited,
                    "loop" => AdaTokenType::Loop,
                    "mod" => AdaTokenType::Mod,
                    "new" => AdaTokenType::New,
                    "not" => AdaTokenType::Not,
                    "null" => AdaTokenType::Null,
                    "of" => AdaTokenType::Of,
                    "or" => AdaTokenType::Or,
                    "others" => AdaTokenType::Others,
                    "out" => AdaTokenType::Out,
                    "overriding" => AdaTokenType::Overriding,
                    "package" => AdaTokenType::Package,
                    "pragma" => AdaTokenType::Pragma,
                    "private" => AdaTokenType::Private,
                    "procedure" => AdaTokenType::Procedure,
                    "protected" => AdaTokenType::Protected,
                    "raise" => AdaTokenType::Raise,
                    "range" => AdaTokenType::Range,
                    "record" => AdaTokenType::Record,
                    "rem" => AdaTokenType::Rem,
                    "renames" => AdaTokenType::Renames,
                    "requeue" => AdaTokenType::Requeue,
                    "return" => AdaTokenType::Return,
                    "reverse" => AdaTokenType::Reverse,
                    "select" => AdaTokenType::Select,
                    "separate" => AdaTokenType::Separate,
                    "some" => AdaTokenType::Some,
                    "subtype" => AdaTokenType::Subtype,
                    "synchronized" => AdaTokenType::Synchronized,
                    "tagged" => AdaTokenType::Tagged,
                    "task" => AdaTokenType::Task,
                    "terminate" => AdaTokenType::Terminate,
                    "then" => AdaTokenType::Then,
                    "type" => AdaTokenType::Type,
                    "until" => AdaTokenType::Until,
                    "use" => AdaTokenType::Use,
                    "when" => AdaTokenType::When,
                    "while" => AdaTokenType::While,
                    "with" => AdaTokenType::With,
                    "xor" => AdaTokenType::Xor,
                    _ => AdaTokenType::Identifier,
                };

                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Multi-character operators first
        if state.consume_if_starts_with("**") {
            state.add_token(AdaTokenType::StarStar, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("=>") {
            state.add_token(AdaTokenType::Arrow, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<=") {
            state.add_token(AdaTokenType::Le, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with(">=") {
            state.add_token(AdaTokenType::Ge, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with(":=") {
            state.add_token(AdaTokenType::ColonEq, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("..") {
            state.add_token(AdaTokenType::DotDot, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("/=") {
            state.add_token(AdaTokenType::Ne, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<<") {
            state.add_token(AdaTokenType::LtLt, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with(">>") {
            state.add_token(AdaTokenType::GtGt, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<>") {
            state.add_token(AdaTokenType::Box, start, state.get_position());
            return true;
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => AdaTokenType::Plus,
                '-' => AdaTokenType::Minus,
                '*' => AdaTokenType::Star,
                '/' => AdaTokenType::Slash,
                '=' => AdaTokenType::Eq,
                '<' => AdaTokenType::Lt,
                '>' => AdaTokenType::Gt,
                '&' => AdaTokenType::Ampersand,
                '|' => AdaTokenType::Pipe,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => AdaTokenType::LeftParen,
                ')' => AdaTokenType::RightParen,
                '[' => AdaTokenType::LeftBracket,
                ']' => AdaTokenType::RightBracket,
                '{' => AdaTokenType::LeftBrace,
                '}' => AdaTokenType::RightBrace,
                ',' => AdaTokenType::Comma,
                ';' => AdaTokenType::Semicolon,
                ':' => AdaTokenType::Colon,
                '.' => AdaTokenType::Dot,
                '\'' => AdaTokenType::Apostrophe,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
