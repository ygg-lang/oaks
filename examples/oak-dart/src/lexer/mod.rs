#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::DartLanguage, lexer::token_type::DartTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, DartLanguage>;

/// Lexer implementation for Dart language
#[derive(Clone)]
pub struct DartLexer<'config> {
    _config: &'config DartLanguage,
}

impl<'config> Lexer<DartLanguage> for DartLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<DartLanguage>) -> LexOutput<DartLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DartLexer<'config> {
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_literal(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // Fallback for unknown characters
            let start = state.get_position();
            if let Some(_ch) = state.bump() {
                state.add_token(DartTokenType::Error, start, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.bump();
            }
            else {
                break;
            }
        }
        if state.get_position() > start {
            state.add_token(DartTokenType::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("///") {
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.bump();
            }
            state.add_token(DartTokenType::DocComment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("//") {
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.bump();
            }
            state.add_token(DartTokenType::LineComment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("/*") {
            while let Some(_ch) = state.peek() {
                if state.consume_if_starts_with("*/") {
                    break;
                }
                state.bump();
            }
            state.add_token(DartTokenType::BlockComment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.bump();
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = match text.as_ref() {
                    "abstract" => DartTokenType::Abstract,
                    "as" => DartTokenType::As,
                    "assert" => DartTokenType::Assert,
                    "async" => DartTokenType::Async,
                    "await" => DartTokenType::Await,
                    "break" => DartTokenType::Break,
                    "case" => DartTokenType::Case,
                    "catch" => DartTokenType::Catch,
                    "class" => DartTokenType::Class,
                    "const" => DartTokenType::Const,
                    "continue" => DartTokenType::Continue,
                    "covariant" => DartTokenType::Covariant,
                    "default" => DartTokenType::Default,
                    "deferred" => DartTokenType::Deferred,
                    "do" => DartTokenType::Do,
                    "dynamic" => DartTokenType::Dynamic,
                    "else" => DartTokenType::Else,
                    "enum" => DartTokenType::Enum,
                    "export" => DartTokenType::Export,
                    "extends" => DartTokenType::Extends,
                    "extension" => DartTokenType::Extension,
                    "external" => DartTokenType::External,
                    "factory" => DartTokenType::Factory,
                    "false" => DartTokenType::False,
                    "final" => DartTokenType::Final,
                    "finally" => DartTokenType::Finally,
                    "for" => DartTokenType::For,
                    "Function" => DartTokenType::Function,
                    "get" => DartTokenType::Get,
                    "hide" => DartTokenType::Hide,
                    "if" => DartTokenType::If,
                    "implements" => DartTokenType::Implements,
                    "import" => DartTokenType::Import,
                    "in" => DartTokenType::In,
                    "interface" => DartTokenType::Interface,
                    "int" => DartTokenType::Int,
                    "is" => DartTokenType::Is,
                    "late" => DartTokenType::Late,
                    "library" => DartTokenType::Library,
                    "mixin" => DartTokenType::Mixin,
                    "new" => DartTokenType::New,
                    "null" => DartTokenType::Null,
                    "on" => DartTokenType::On,
                    "operator" => DartTokenType::Operator,
                    "part" => DartTokenType::Part,
                    "required" => DartTokenType::Required,
                    "rethrow" => DartTokenType::Rethrow,
                    "return" => DartTokenType::Return,
                    "set" => DartTokenType::Set,
                    "show" => DartTokenType::Show,
                    "static" => DartTokenType::Static,
                    "super" => DartTokenType::Super,
                    "switch" => DartTokenType::Switch,
                    "sync" => DartTokenType::Sync,
                    "this" => DartTokenType::This,
                    "throw" => DartTokenType::Throw,
                    "true" => DartTokenType::True,
                    "try" => DartTokenType::Try,
                    "typedef" => DartTokenType::Typedef,
                    "var" => DartTokenType::Var,
                    "void" => DartTokenType::Void,
                    "while" => DartTokenType::While,
                    "with" => DartTokenType::With,
                    "yield" => DartTokenType::Yield,
                    _ => DartTokenType::Identifier,
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }

    fn lex_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.bump();
                let mut is_double = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.bump();
                    }
                    else if ch == '.' && !is_double {
                        is_double = true;
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                let kind = if is_double { DartTokenType::DoubleLiteral } else { DartTokenType::IntegerLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
            if ch == '\'' || ch == '"' {
                let quote = ch;
                state.bump();
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.bump();
                        break;
                    }
                    if ch == '\\' {
                        state.bump();
                        state.bump();
                    }
                    else {
                        state.bump();
                    }
                }
                state.add_token(DartTokenType::StringLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let kind = if state.consume_if_starts_with("...") {
            Some(DartTokenType::DotDotDot)
        }
        else if state.consume_if_starts_with("..") {
            Some(DartTokenType::DotDot)
        }
        else if state.consume_if_starts_with("??=") {
            Some(DartTokenType::QuestionQuestionEqual)
        }
        else if state.consume_if_starts_with("??") {
            Some(DartTokenType::QuestionQuestion)
        }
        else if state.consume_if_starts_with("?.") {
            Some(DartTokenType::QuestionDot)
        }
        else if state.consume_if_starts_with("~//") {
            // This looks like a mistake in DartTokenType or my understanding,
            // but let's follow the most likely ones.
            None
        }
        else if state.consume_if_starts_with("~/=") {
            Some(DartTokenType::TildeSlashEqual)
        }
        else if state.consume_if_starts_with("~/") {
            Some(DartTokenType::TildeSlash)
        }
        else if state.consume_if_starts_with("<<=") {
            Some(DartTokenType::LeftShiftEqual)
        }
        else if state.consume_if_starts_with("<<") {
            Some(DartTokenType::LeftShift)
        }
        else if state.consume_if_starts_with(">>=") {
            Some(DartTokenType::RightShiftEqual)
        }
        else if state.consume_if_starts_with(">>") {
            Some(DartTokenType::RightShift)
        }
        else if state.consume_if_starts_with("==") {
            Some(DartTokenType::EqualEqual)
        }
        else if state.consume_if_starts_with("!=") {
            Some(DartTokenType::BangEqual)
        }
        else if state.consume_if_starts_with("<=") {
            Some(DartTokenType::LessEqual)
        }
        else if state.consume_if_starts_with(">=") {
            Some(DartTokenType::GreaterEqual)
        }
        else if state.consume_if_starts_with("&&") {
            Some(DartTokenType::AmpersandAmpersand)
        }
        else if state.consume_if_starts_with("||") {
            Some(DartTokenType::PipePipe)
        }
        else if state.consume_if_starts_with("++") {
            Some(DartTokenType::PlusPlus)
        }
        else if state.consume_if_starts_with("--") {
            Some(DartTokenType::MinusMinus)
        }
        else if state.consume_if_starts_with("+=") {
            Some(DartTokenType::PlusEqual)
        }
        else if state.consume_if_starts_with("-=") {
            Some(DartTokenType::MinusEqual)
        }
        else if state.consume_if_starts_with("*=") {
            Some(DartTokenType::StarEqual)
        }
        else if state.consume_if_starts_with("/=") {
            Some(DartTokenType::SlashEqual)
        }
        else if state.consume_if_starts_with("%=") {
            Some(DartTokenType::PercentEqual)
        }
        else if state.consume_if_starts_with("&=") {
            Some(DartTokenType::AmpersandEqual)
        }
        else if state.consume_if_starts_with("|=") {
            Some(DartTokenType::PipeEqual)
        }
        else if state.consume_if_starts_with("^=") {
            Some(DartTokenType::CaretEqual)
        }
        else if state.consume_if_starts_with("=>") {
            Some(DartTokenType::Arrow)
        }
        else {
            match state.peek() {
                Some('+') => Some(DartTokenType::Plus),
                Some('-') => Some(DartTokenType::Minus),
                Some('*') => Some(DartTokenType::Star),
                Some('/') => Some(DartTokenType::Slash),
                Some('%') => Some(DartTokenType::Percent),
                Some('=') => Some(DartTokenType::Equal),
                Some('<') => Some(DartTokenType::Less),
                Some('>') => Some(DartTokenType::Greater),
                Some('&') => Some(DartTokenType::Ampersand),
                Some('|') => Some(DartTokenType::Pipe),
                Some('^') => Some(DartTokenType::Caret),
                Some('~') => Some(DartTokenType::Tilde),
                Some('!') => Some(DartTokenType::Bang),
                Some('?') => Some(DartTokenType::Question),
                Some('.') => Some(DartTokenType::Dot),
                Some('(') => Some(DartTokenType::LeftParen),
                Some(')') => Some(DartTokenType::RightParen),
                Some('[') => Some(DartTokenType::LeftBracket),
                Some(']') => Some(DartTokenType::RightBracket),
                Some('{') => Some(DartTokenType::LeftBrace),
                Some('}') => Some(DartTokenType::RightBrace),
                Some(';') => Some(DartTokenType::Semicolon),
                Some(',') => Some(DartTokenType::Comma),
                Some(':') => Some(DartTokenType::Colon),
                Some('@') => Some(DartTokenType::At),
                Some('#') => Some(DartTokenType::Hash),
                _ => None,
            }
        };

        if let Some(kind) = kind {
            if state.get_position() == start {
                state.bump();
            }
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
