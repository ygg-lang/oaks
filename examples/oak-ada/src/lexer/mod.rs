use crate::{kind::AdaSyntaxKind, language::AdaLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, AdaLanguage>;

static ADA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ADA_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["--"] });
static ADA_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: None });
static ADA_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct AdaLexer<'config> {
    config: &'config AdaLanguage,
}

impl<'config> Lexer<AdaLanguage> for AdaLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<AdaLanguage>,
    ) -> LexOutput<AdaLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> AdaLexer<'config> {
    pub fn new(config: &'config AdaLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
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

            // 如果没有匹配任何模式，跳过当前字符并生成 Error token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(AdaSyntaxKind::Error, safe_point, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(AdaSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match ADA_WHITESPACE.scan(state.rest(), state.get_position(), AdaSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Ada line comment: -- ... until newline
        if rest.starts_with("--") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(AdaSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // Ada string: "..."
        if state.current() == Some('"') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(AdaSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('\'') {
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
            state.add_token(AdaSyntaxKind::CharacterLiteral, start, state.get_position());
            return true;
        }
        state.set_position(start);
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                // consume digits
                state.advance(ch.len_utf8());
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // check for decimal point
                if state.current() == Some('.') {
                    state.advance(1);
                    while let Some(ch) = state.current() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // check for exponent
                if let Some(ch) = state.current() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.current() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.current() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(AdaSyntaxKind::NumberLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = match text.to_lowercase().as_str() {
                    "abort" => AdaSyntaxKind::AbortKeyword,
                    "abs" => AdaSyntaxKind::AbsKeyword,
                    "abstract" => AdaSyntaxKind::AbstractKeyword,
                    "accept" => AdaSyntaxKind::AcceptKeyword,
                    "access" => AdaSyntaxKind::AccessKeyword,
                    "aliased" => AdaSyntaxKind::AliasedKeyword,
                    "all" => AdaSyntaxKind::AllKeyword,
                    "and" => AdaSyntaxKind::AndKeyword,
                    "array" => AdaSyntaxKind::ArrayKeyword,
                    "at" => AdaSyntaxKind::AtKeyword,
                    "begin" => AdaSyntaxKind::BeginKeyword,
                    "body" => AdaSyntaxKind::BodyKeyword,
                    "case" => AdaSyntaxKind::CaseKeyword,
                    "constant" => AdaSyntaxKind::ConstantKeyword,
                    "declare" => AdaSyntaxKind::DeclareKeyword,
                    "delay" => AdaSyntaxKind::DelayKeyword,
                    "delta" => AdaSyntaxKind::DeltaKeyword,
                    "digits" => AdaSyntaxKind::DigitsKeyword,
                    "do" => AdaSyntaxKind::DoKeyword,
                    "else" => AdaSyntaxKind::ElseKeyword,
                    "elsif" => AdaSyntaxKind::ElsifKeyword,
                    "end" => AdaSyntaxKind::EndKeyword,
                    "entry" => AdaSyntaxKind::EntryKeyword,
                    "exception" => AdaSyntaxKind::ExceptionKeyword,
                    "exit" => AdaSyntaxKind::ExitKeyword,
                    "for" => AdaSyntaxKind::ForKeyword,
                    "function" => AdaSyntaxKind::FunctionKeyword,
                    "generic" => AdaSyntaxKind::GenericKeyword,
                    "goto" => AdaSyntaxKind::GotoKeyword,
                    "if" => AdaSyntaxKind::IfKeyword,
                    "in" => AdaSyntaxKind::InKeyword,
                    "interface" => AdaSyntaxKind::InterfaceKeyword,
                    "is" => AdaSyntaxKind::IsKeyword,
                    "limited" => AdaSyntaxKind::LimitedKeyword,
                    "loop" => AdaSyntaxKind::LoopKeyword,
                    "mod" => AdaSyntaxKind::ModKeyword,
                    "new" => AdaSyntaxKind::NewKeyword,
                    "not" => AdaSyntaxKind::NotKeyword,
                    "null" => AdaSyntaxKind::NullKeyword,
                    "of" => AdaSyntaxKind::OfKeyword,
                    "or" => AdaSyntaxKind::OrKeyword,
                    "others" => AdaSyntaxKind::OthersKeyword,
                    "out" => AdaSyntaxKind::OutKeyword,
                    "overriding" => AdaSyntaxKind::OverridingKeyword,
                    "package" => AdaSyntaxKind::PackageKeyword,
                    "pragma" => AdaSyntaxKind::PragmaKeyword,
                    "private" => AdaSyntaxKind::PrivateKeyword,
                    "procedure" => AdaSyntaxKind::ProcedureKeyword,
                    "protected" => AdaSyntaxKind::ProtectedKeyword,
                    "raise" => AdaSyntaxKind::RaiseKeyword,
                    "range" => AdaSyntaxKind::RangeKeyword,
                    "record" => AdaSyntaxKind::RecordKeyword,
                    "rem" => AdaSyntaxKind::RemKeyword,
                    "renames" => AdaSyntaxKind::RenamesKeyword,
                    "requeue" => AdaSyntaxKind::RequeueKeyword,
                    "return" => AdaSyntaxKind::ReturnKeyword,
                    "reverse" => AdaSyntaxKind::ReverseKeyword,
                    "select" => AdaSyntaxKind::SelectKeyword,
                    "separate" => AdaSyntaxKind::SeparateKeyword,
                    "subtype" => AdaSyntaxKind::SubtypeKeyword,
                    "synchronized" => AdaSyntaxKind::SynchronizedKeyword,
                    "tagged" => AdaSyntaxKind::TaggedKeyword,
                    "task" => AdaSyntaxKind::TaskKeyword,
                    "terminate" => AdaSyntaxKind::TerminateKeyword,
                    "then" => AdaSyntaxKind::ThenKeyword,
                    "type" => AdaSyntaxKind::TypeKeyword,
                    "until" => AdaSyntaxKind::UntilKeyword,
                    "use" => AdaSyntaxKind::UseKeyword,
                    "when" => AdaSyntaxKind::WhenKeyword,
                    "while" => AdaSyntaxKind::WhileKeyword,
                    "with" => AdaSyntaxKind::WithKeyword,
                    "xor" => AdaSyntaxKind::XorKeyword,
                    _ => AdaSyntaxKind::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Multi-character operators first
        if rest.starts_with("**") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::DoubleStar, start, state.get_position());
            return true;
        }
        if rest.starts_with("=>") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::Arrow, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::LessEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::GreaterEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(":=") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::ColonEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("..") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::DotDot, start, state.get_position());
            return true;
        }
        if rest.starts_with("/=") {
            state.advance(2);
            state.add_token(AdaSyntaxKind::NotEqual, start, state.get_position());
            return true;
        }

        // Single-character operators
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => AdaSyntaxKind::Plus,
                '-' => AdaSyntaxKind::Minus,
                '*' => AdaSyntaxKind::Star,
                '/' => AdaSyntaxKind::Slash,
                '=' => AdaSyntaxKind::Equal,
                '<' => AdaSyntaxKind::Less,
                '>' => AdaSyntaxKind::Greater,
                '&' => AdaSyntaxKind::Ampersand,
                '|' => AdaSyntaxKind::Pipe,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => AdaSyntaxKind::LeftParen,
                ')' => AdaSyntaxKind::RightParen,
                '[' => AdaSyntaxKind::LeftBracket,
                ']' => AdaSyntaxKind::RightBracket,
                '{' => AdaSyntaxKind::LeftBrace,
                '}' => AdaSyntaxKind::RightBrace,
                ',' => AdaSyntaxKind::Comma,
                ';' => AdaSyntaxKind::Semicolon,
                ':' => AdaSyntaxKind::Colon,
                '.' => AdaSyntaxKind::Dot,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
