pub mod token_type;

pub use token_type::ActionScriptTokenType;

use crate::language::ActionScriptLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
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
        AS_CHAR.scan(state, ActionScriptTokenType::CharLiteral)
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' || c == '$' => c,
            _ => return false,
        };

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
        let text = state.get_text_in((start..end).into());
        let kind = match text.as_ref() {
            "as" => ActionScriptTokenType::As,
            "break" => ActionScriptTokenType::Break,
            "case" => ActionScriptTokenType::Case,
            "catch" => ActionScriptTokenType::Catch,
            "class" => ActionScriptTokenType::Class,
            "const" => ActionScriptTokenType::Const,
            "continue" => ActionScriptTokenType::Continue,
            "default" => ActionScriptTokenType::Default,
            "delete" => ActionScriptTokenType::Delete,
            "do" => ActionScriptTokenType::Do,
            "else" => ActionScriptTokenType::Else,
            "extends" => ActionScriptTokenType::Extends,
            "false" => ActionScriptTokenType::False,
            "finally" => ActionScriptTokenType::Finally,
            "for" => ActionScriptTokenType::For,
            "function" => ActionScriptTokenType::Function,
            "if" => ActionScriptTokenType::If,
            "implements" => ActionScriptTokenType::Implements,
            "import" => ActionScriptTokenType::Import,
            "in" => ActionScriptTokenType::In,
            "instanceof" => ActionScriptTokenType::Instanceof,
            "interface" => ActionScriptTokenType::Interface,
            "internal" => ActionScriptTokenType::Internal,
            "is" => ActionScriptTokenType::Is,
            "native" => ActionScriptTokenType::Native,
            "new" => ActionScriptTokenType::New,
            "null" => ActionScriptTokenType::Null,
            "package" => ActionScriptTokenType::Package,
            "private" => ActionScriptTokenType::Private,
            "protected" => ActionScriptTokenType::Protected,
            "public" => ActionScriptTokenType::Public,
            "return" => ActionScriptTokenType::Return,
            "static" => ActionScriptTokenType::Static,
            "super" => ActionScriptTokenType::Super,
            "switch" => ActionScriptTokenType::Switch,
            "this" => ActionScriptTokenType::This,
            "throw" => ActionScriptTokenType::Throw,
            "true" => ActionScriptTokenType::True,
            "try" => ActionScriptTokenType::Try,
            "typeof" => ActionScriptTokenType::Typeof,
            "use" => ActionScriptTokenType::Use,
            "var" => ActionScriptTokenType::Var,
            "void" => ActionScriptTokenType::Void,
            "while" => ActionScriptTokenType::While,
            "with" => ActionScriptTokenType::With,
            "each" => ActionScriptTokenType::Each,
            "get" => ActionScriptTokenType::Get,
            "set" => ActionScriptTokenType::Set,
            "namespace" => ActionScriptTokenType::Namespace,
            "include" => ActionScriptTokenType::Include,
            "dynamic" => ActionScriptTokenType::Dynamic,
            "final" => ActionScriptTokenType::Final,
            "override" => ActionScriptTokenType::Override,
            "Array" => ActionScriptTokenType::Array,
            "Boolean" => ActionScriptTokenType::Boolean,
            "Date" => ActionScriptTokenType::Date,
            "Number" => ActionScriptTokenType::Number,
            "Object" => ActionScriptTokenType::ObjectType,
            "RegExp" => ActionScriptTokenType::RegExp,
            "String" => ActionScriptTokenType::StringType,
            "uint" => ActionScriptTokenType::Uint,
            "Vector" => ActionScriptTokenType::Vector,
            "XML" => ActionScriptTokenType::Xml,
            "XMLList" => ActionScriptTokenType::XmlList,
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
