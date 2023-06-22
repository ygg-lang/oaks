use crate::{kind::DartSyntaxKind, language::DartLanguage};
use oak_core::{
    LexOutput, Lexer, LexerCache, LexerState, OakError, Source, TextEdit,
    lexer::{CommentConfig, StringConfig, WhitespaceConfig},
};

type State<'a, S> = LexerState<'a, S, DartLanguage>;

static DART_WHITESPACE: WhitespaceConfig = WhitespaceConfig { unicode_whitespace: true };
static DART_COMMENT: CommentConfig = CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true };
static DART_STRING_DOUBLE: StringConfig = StringConfig { quotes: &['"'], escape: Some('\\') };
static DART_STRING_SINGLE: StringConfig = StringConfig { quotes: &['\''], escape: Some('\\') };

/// Lexer implementation for Dart language
#[derive(Clone)]
pub struct DartLexer<'config> {
    _config: &'config DartLanguage,
}

impl<'config> Lexer<DartLanguage> for DartLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DartLanguage>) -> LexOutput<DartLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
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

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        DART_WHITESPACE.scan(state, DartSyntaxKind::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        DART_COMMENT.scan(state, DartSyntaxKind::LineComment, DartSyntaxKind::BlockComment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if DART_STRING_DOUBLE.scan(state, DartSyntaxKind::StringLiteral) {
            return true;
        }
        if DART_STRING_SINGLE.scan(state, DartSyntaxKind::StringLiteral) {
            return true;
        }
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 消费数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let mut is_double = false;

                // 检查小数点
                if state.starts_with(".") && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // 跳过 '.'
                    is_double = true;

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        is_double = true;

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

                let kind = if is_double { DartSyntaxKind::DoubleLiteral } else { DartSyntaxKind::IntegerLiteral };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '$') {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        let kind = match text.as_ref() {
            "abstract" => DartSyntaxKind::Abstract,
            "as" => DartSyntaxKind::As,
            "assert" => DartSyntaxKind::Assert,
            "async" => DartSyntaxKind::Async,
            "await" => DartSyntaxKind::Await,
            "break" => DartSyntaxKind::Break,
            "case" => DartSyntaxKind::Case,
            "catch" => DartSyntaxKind::Catch,
            "class" => DartSyntaxKind::Class,
            "const" => DartSyntaxKind::Const,
            "continue" => DartSyntaxKind::Continue,
            "covariant" => DartSyntaxKind::Covariant,
            "default" => DartSyntaxKind::Default,
            "deferred" => DartSyntaxKind::Deferred,
            "do" => DartSyntaxKind::Do,
            "dynamic" => DartSyntaxKind::Dynamic,
            "else" => DartSyntaxKind::Else,
            "enum" => DartSyntaxKind::Enum,
            "export" => DartSyntaxKind::Export,
            "extends" => DartSyntaxKind::Extends,
            "extension" => DartSyntaxKind::Extension,
            "external" => DartSyntaxKind::External,
            "factory" => DartSyntaxKind::Factory,
            "false" => DartSyntaxKind::False,
            "final" => DartSyntaxKind::Final,
            "finally" => DartSyntaxKind::Finally,
            "for" => DartSyntaxKind::For,
            "function" => DartSyntaxKind::Function,
            "get" => DartSyntaxKind::Get,
            "hide" => DartSyntaxKind::Hide,
            "if" => DartSyntaxKind::If,
            "implements" => DartSyntaxKind::Implements,
            "import" => DartSyntaxKind::Import,
            "in" => DartSyntaxKind::In,
            "interface" => DartSyntaxKind::Interface,
            "is" => DartSyntaxKind::Is,
            "late" => DartSyntaxKind::Late,
            "library" => DartSyntaxKind::Library,
            "mixin" => DartSyntaxKind::Mixin,
            "new" => DartSyntaxKind::New,
            "null" => DartSyntaxKind::Null,
            "on" => DartSyntaxKind::On,
            "operator" => DartSyntaxKind::Operator,
            "part" => DartSyntaxKind::Part,
            "required" => DartSyntaxKind::Required,
            "rethrow" => DartSyntaxKind::Rethrow,
            "return" => DartSyntaxKind::Return,
            "set" => DartSyntaxKind::Set,
            "show" => DartSyntaxKind::Show,
            "static" => DartSyntaxKind::Static,
            "super" => DartSyntaxKind::Super,
            "switch" => DartSyntaxKind::Switch,
            "sync" => DartSyntaxKind::Sync,
            "this" => DartSyntaxKind::This,
            "throw" => DartSyntaxKind::Throw,
            "true" => DartSyntaxKind::True,
            "try" => DartSyntaxKind::Try,
            "typedef" => DartSyntaxKind::Typedef,
            "var" => DartSyntaxKind::Var,
            "void" => DartSyntaxKind::Void,
            "while" => DartSyntaxKind::While,
            "with" => DartSyntaxKind::With,
            "yield" => DartSyntaxKind::Yield,
            _ => DartSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        let kinds = [
            ("??=", DartSyntaxKind::QuestionQuestionEqual),
            ("??", DartSyntaxKind::QuestionQuestion),
            ("&&", DartSyntaxKind::AmpersandAmpersand),
            ("||", DartSyntaxKind::PipePipe),
            ("==", DartSyntaxKind::EqualEqual),
            ("!=", DartSyntaxKind::BangEqual),
            (">=", DartSyntaxKind::GreaterEqual),
            ("<=", DartSyntaxKind::LessEqual),
            ("+=", DartSyntaxKind::PlusEqual),
            ("-=", DartSyntaxKind::MinusEqual),
            ("*=", DartSyntaxKind::StarEqual),
            ("/=", DartSyntaxKind::SlashEqual),
            ("%=", DartSyntaxKind::PercentEqual),
            ("&=", DartSyntaxKind::AmpersandEqual),
            ("|=", DartSyntaxKind::PipeEqual),
            ("^=", DartSyntaxKind::CaretEqual),
            ("~/=", DartSyntaxKind::TildeSlashEqual),
            ("~/", DartSyntaxKind::TildeSlash),
            ("<<=", DartSyntaxKind::LeftShiftEqual),
            (">>=", DartSyntaxKind::RightShiftEqual),
            ("<<", DartSyntaxKind::LeftShift),
            (">>", DartSyntaxKind::RightShift),
            ("=>", DartSyntaxKind::Arrow),
            ("...", DartSyntaxKind::DotDotDot),
            ("..", DartSyntaxKind::DotDot),
            ("?.", DartSyntaxKind::QuestionDot),
            ("++", DartSyntaxKind::PlusPlus),
            ("--", DartSyntaxKind::MinusMinus),
        ];

        for (op, kind) in kinds {
            if state.consume_if_starts_with(op) {
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        let kind = match ch {
            '(' => DartSyntaxKind::LeftParen,
            ')' => DartSyntaxKind::RightParen,
            '{' => DartSyntaxKind::LeftBrace,
            '}' => DartSyntaxKind::RightBrace,
            '[' => DartSyntaxKind::LeftBracket,
            ']' => DartSyntaxKind::RightBracket,
            ';' => DartSyntaxKind::Semicolon,
            ',' => DartSyntaxKind::Comma,
            '.' => DartSyntaxKind::Dot,
            ':' => DartSyntaxKind::Colon,
            '?' => DartSyntaxKind::Question,
            '=' => DartSyntaxKind::Equal,
            '!' => DartSyntaxKind::Bang,
            '>' => DartSyntaxKind::Greater,
            '<' => DartSyntaxKind::Less,
            '+' => DartSyntaxKind::Plus,
            '-' => DartSyntaxKind::Minus,
            '*' => DartSyntaxKind::Star,
            '/' => DartSyntaxKind::Slash,
            '%' => DartSyntaxKind::Percent,
            '&' => DartSyntaxKind::Ampersand,
            '|' => DartSyntaxKind::Pipe,
            '^' => DartSyntaxKind::Caret,
            '~' => DartSyntaxKind::Tilde,
            '@' => DartSyntaxKind::At,
            '#' => DartSyntaxKind::Hash,
            _ => return false,
        };

        state.advance(ch.len_utf8());
        state.add_token(kind, start, state.get_position());
        true
    }
}
