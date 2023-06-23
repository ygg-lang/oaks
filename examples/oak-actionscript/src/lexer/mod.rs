#![doc = include_str!("readme.md")]
use oak_core::Source;
pub mod token_type;

pub use token_type::ActionScriptTokenType;

use crate::language::ActionScriptLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, ActionScriptLanguage>;

static AS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static AS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static AS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static AS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ActionScriptLexer<'config> {
    _config: &'config ActionScriptLanguage,
}

impl<'config> Lexer<ActionScriptLanguage> for ActionScriptLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<ActionScriptLanguage>) -> LexOutput<ActionScriptLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ActionScriptLexer<'config> {
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { _config: config }
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

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        AS_WHITESPACE.scan(state, ActionScriptTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        AS_COMMENT.scan(state, ActionScriptTokenType::Comment, ActionScriptTokenType::Comment)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(first.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }
        state.add_token(ActionScriptTokenType::NumberLiteral, start, state.get_position());
        true
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        AS_STRING.scan(state, ActionScriptTokenType::StringLiteral)
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        AS_CHAR.scan(state, ActionScriptTokenType::StringLiteral)
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_alphabetic() && first != '_' && first != '$' {
            return false;
        }

        state.advance(first.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.source().get_text_in(oak_core::Range { start, end });
        let kind = match text.as_ref() {
            "class" => ActionScriptTokenType::Class,
            "interface" => ActionScriptTokenType::Interface,
            "function" => ActionScriptTokenType::Function,
            "var" => ActionScriptTokenType::Var,
            "const" => ActionScriptTokenType::Const,
            "public" => ActionScriptTokenType::Public,
            "private" => ActionScriptTokenType::Private,
            "protected" => ActionScriptTokenType::Protected,
            "internal" => ActionScriptTokenType::Internal,
            "static" => ActionScriptTokenType::Static,
            "override" => ActionScriptTokenType::Override,
            "package" => ActionScriptTokenType::Package,
            "import" => ActionScriptTokenType::Import,
            "extends" => ActionScriptTokenType::Extends,
            "implements" => ActionScriptTokenType::Implements,
            "new" => ActionScriptTokenType::New,
            "this" => ActionScriptTokenType::This,
            "super" => ActionScriptTokenType::Super,
            "if" => ActionScriptTokenType::If,
            "else" => ActionScriptTokenType::Else,
            "for" => ActionScriptTokenType::For,
            "while" => ActionScriptTokenType::While,
            "do" => ActionScriptTokenType::Do,
            "switch" => ActionScriptTokenType::Switch,
            "case" => ActionScriptTokenType::Case,
            "default" => ActionScriptTokenType::Default,
            "break" => ActionScriptTokenType::Break,
            "continue" => ActionScriptTokenType::Continue,
            "return" => ActionScriptTokenType::Return,
            "try" => ActionScriptTokenType::Try,
            "catch" => ActionScriptTokenType::Catch,
            "finally" => ActionScriptTokenType::Finally,
            "throw" => ActionScriptTokenType::Throw,
            "void" => ActionScriptTokenType::Void,
            "null" => ActionScriptTokenType::Null,
            "true" => ActionScriptTokenType::True,
            "false" => ActionScriptTokenType::False,
            _ => ActionScriptTokenType::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let c = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        let kind = match c {
            '+' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::PlusAssign
                    }
                    Some('+') => {
                        state.advance(1);
                        ActionScriptTokenType::Increment
                    }
                    _ => ActionScriptTokenType::Plus,
                }
            }
            '-' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::MinusAssign
                    }
                    Some('-') => {
                        state.advance(1);
                        ActionScriptTokenType::Decrement
                    }
                    Some('>') => {
                        state.advance(1);
                        ActionScriptTokenType::Arrow
                    }
                    _ => ActionScriptTokenType::Minus,
                }
            }
            '*' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::StarAssign
                    }
                    _ => ActionScriptTokenType::Star,
                }
            }
            '/' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::SlashAssign
                    }
                    _ => ActionScriptTokenType::Slash,
                }
            }
            '%' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::PercentAssign
                    }
                    _ => ActionScriptTokenType::Percent,
                }
            }
            '=' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        match state.peek() {
                            Some('=') => {
                                state.advance(1);
                                ActionScriptTokenType::EqualEqualEqual
                            }
                            _ => ActionScriptTokenType::EqualEqual,
                        }
                    }
                    _ => ActionScriptTokenType::Equal,
                }
            }
            '!' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        match state.peek() {
                            Some('=') => {
                                state.advance(1);
                                ActionScriptTokenType::NotEqualEqual
                            }
                            _ => ActionScriptTokenType::NotEqual,
                        }
                    }
                    _ => ActionScriptTokenType::LogicalNot,
                }
            }
            '<' => {
                state.advance(1);
                match state.peek() {
                    Some('<') => {
                        state.advance(1);
                        match state.peek() {
                            Some('=') => {
                                state.advance(1);
                                ActionScriptTokenType::LeftShiftAssign
                            }
                            _ => ActionScriptTokenType::LeftShift,
                        }
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::LessEqual
                    }
                    _ => ActionScriptTokenType::LessThan,
                }
            }
            '>' => {
                state.advance(1);
                match state.peek() {
                    Some('>') => {
                        state.advance(1);
                        match state.peek() {
                            Some('>') => {
                                state.advance(1);
                                match state.peek() {
                                    Some('=') => {
                                        state.advance(1);
                                        ActionScriptTokenType::UnsignedRightShiftAssign
                                    }
                                    _ => ActionScriptTokenType::UnsignedRightShift,
                                }
                            }
                            Some('=') => {
                                state.advance(1);
                                ActionScriptTokenType::RightShiftAssign
                            }
                            _ => ActionScriptTokenType::RightShift,
                        }
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::GreaterEqual
                    }
                    _ => ActionScriptTokenType::GreaterThan,
                }
            }
            '&' => {
                state.advance(1);
                match state.peek() {
                    Some('&') => {
                        state.advance(1);
                        ActionScriptTokenType::LogicalAnd
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::BitwiseAndAssign
                    }
                    _ => ActionScriptTokenType::BitwiseAnd,
                }
            }
            '|' => {
                state.advance(1);
                match state.peek() {
                    Some('|') => {
                        state.advance(1);
                        ActionScriptTokenType::LogicalOr
                    }
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::BitwiseOrAssign
                    }
                    _ => ActionScriptTokenType::BitwiseOr,
                }
            }
            '^' => {
                state.advance(1);
                match state.peek() {
                    Some('=') => {
                        state.advance(1);
                        ActionScriptTokenType::BitwiseXorAssign
                    }
                    _ => ActionScriptTokenType::BitwiseXor,
                }
            }
            '~' => {
                state.advance(1);
                ActionScriptTokenType::BitwiseNot
            }
            '?' => {
                state.advance(1);
                ActionScriptTokenType::Question
            }
            ':' => {
                state.advance(1);
                ActionScriptTokenType::Colon
            }
            '.' => {
                state.advance(1);
                ActionScriptTokenType::Dot
            }
            '(' => {
                state.advance(1);
                ActionScriptTokenType::LeftParen
            }
            ')' => {
                state.advance(1);
                ActionScriptTokenType::RightParen
            }
            '{' => {
                state.advance(1);
                ActionScriptTokenType::LeftBrace
            }
            '}' => {
                state.advance(1);
                ActionScriptTokenType::RightBrace
            }
            '[' => {
                state.advance(1);
                ActionScriptTokenType::LeftBracket
            }
            ']' => {
                state.advance(1);
                ActionScriptTokenType::RightBracket
            }
            ';' => {
                state.advance(1);
                ActionScriptTokenType::Semicolon
            }
            ',' => {
                state.advance(1);
                ActionScriptTokenType::Comma
            }
            '@' => {
                state.advance(1);
                ActionScriptTokenType::At
            }
            '#' => {
                state.advance(1);
                ActionScriptTokenType::Hash
            }
            '$' => {
                state.advance(1);
                ActionScriptTokenType::Dollar
            }
            '\\' => {
                state.advance(1);
                ActionScriptTokenType::Backslash
            }
            '\'' => {
                state.advance(1);
                ActionScriptTokenType::Quote
            }
            '"' => {
                state.advance(1);
                ActionScriptTokenType::DoubleQuote
            }
            '`' => {
                state.advance(1);
                ActionScriptTokenType::Backtick
            }
            _ => return false,
        };

        state.add_token(kind, start, state.get_position());
        true
    }
}
